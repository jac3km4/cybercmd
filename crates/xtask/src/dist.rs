use std::{env::set_current_dir, fs};

use anyhow::Result;
use common::file::{download_file, zip_files};
use uniquote::Quote;
use xshell::{cmd, Shell};

use crate::config::Config;

pub fn dist(config: &Config) -> Result<()> {
    let global_ini = config.paths.staging_bin.join("global.ini");
    let version_dll = config.paths.staging_bin.join("version.dll");

    println!("Cleanup staging");
    config.paths.clean_staging()?;

    set_current_dir(config.paths.staging.as_path())?;

    let sh = Shell::new()?;

    println!("Building cybercmd.");
    cmd!(sh, "cargo build --release --package cybercmd").run()?;

    println!("Copying cybercmd.asi");
    sh.copy_file(
        config.paths.release.join("cybercmd.dll"),
        config.paths.staging_plugins.join("cybercmd.asi"),
    )?;

    println!("Adding Vortex install files to fomod/ directory");
    sh.copy_file(
        config.paths.installer.join("info.xml"),
        &config.paths.staging_fomod,
    )?;
    sh.copy_file(
        config.paths.installer.join("Moduleconfig.xml"),
        &config.paths.staging_fomod,
    )?;

    println!("Adding config files (redscript)");
    for path in fs::read_dir(&config.paths.config)? {
        sh.copy_file(path?.path(), &config.paths.staging_config)?;
    }

    println!(
        "Creating: {}",
        &config.paths.release.join("cybercmd.zip").quote()
    );
    zip_files(
        &config.paths.staging,
        config.paths.release.join("cybercmd.zip"),
    )?;

    println!("Downloading global.ini");
    download_file(config.urls.global_ini, global_ini)?;
    println!("Downloading version.dll");
    download_file(config.urls.version_dll, version_dll)?;

    println!(
        "Creating: {}",
        &config.paths.release.join("cybercmd-standalone.zip").quote()
    );
    zip_files(
        &config.paths.staging,
        config.paths.release.join("cybercmd-standalone.zip"),
    )?;

    println!();
    println!("Done!");

    Ok(())
}
