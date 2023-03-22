use std::{env::set_current_dir, fs};

use anyhow::Result;
use uniquote::Quote;
use xshell::{cmd, Shell};

use common::file::{download_file, zip_files};

use crate::config::CONFIG;

pub fn dist() -> Result<()> {
    let global_ini = CONFIG.paths.staging_bin.join("global.ini");
    let version_dll = CONFIG.paths.staging_bin.join("version.dll");

    println!("Cleanup staging");
    CONFIG.paths.clean_staging()?;

    set_current_dir(CONFIG.paths.staging.as_path())?;

    let sh = Shell::new()?;

    println!("Building cybercmd.");
    cmd!(sh, "cargo build --release").run()?;

    println!("Copying cybercmd.asi");
    sh.copy_file(
        CONFIG.paths.release.join("cybercmd.dll"),
        CONFIG.paths.staging_plugins.join("cybercmd.asi"),
    )?;

    println!("Adding Vortex install files to fomod/ directory");
    sh.copy_file(
        CONFIG.paths.installer.join("info.xml"),
        &CONFIG.paths.staging_fomod,
    )?;
    sh.copy_file(
        CONFIG.paths.installer.join("ModuleConfig.xml"),
        &CONFIG.paths.staging_fomod,
    )?;

    println!("Adding config files (redscript)");
    for path in fs::read_dir(&CONFIG.paths.config)? {
        sh.copy_file(path?.path(), &CONFIG.paths.staging_config)?;
    }

    println!(
        "Creating: {}",
        &CONFIG.paths.release.join("cybercmd.zip").quote()
    );
    zip_files(
        &CONFIG.paths.staging,
        CONFIG.paths.release.join("cybercmd.zip"),
    )?;

    println!("Downloading global.ini");
    download_file(CONFIG.urls.global_ini, global_ini)?;
    println!("Downloading version.dll");
    download_file(CONFIG.urls.version_dll, version_dll)?;

    println!(
        "Creating: {}",
        &CONFIG.paths.release.join("cybercmd-standalone.zip").quote()
    );
    zip_files(
        &CONFIG.paths.staging,
        CONFIG.paths.release.join("cybercmd-standalone.zip"),
    )?;

    println!();
    println!("Done!");

    Ok(())
}
