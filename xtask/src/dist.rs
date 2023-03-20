use std::{env::set_current_dir, fs};

use anyhow::Result;
use common::file::{download_file, zip_files};
use uniquote::Quote;
use xshell::{cmd, Shell};

use crate::config::PATHS;

pub fn dist() -> Result<()> {
    let global_ini = PATHS.staging_bin.join("global.ini");
    let version_dll = PATHS.staging_bin.join("version.dll");

    println!("Cleanup staging");
    fs::remove_dir_all(&PATHS.staging)?;
    fs::create_dir_all(&PATHS.staging)?;
    fs::create_dir_all(&PATHS.staging_bin)?;
    fs::create_dir_all(&PATHS.staging_plugins)?;
    fs::create_dir_all(&PATHS.staging_config)?;
    fs::create_dir_all(&PATHS.staging_fomod)?;

    set_current_dir(PATHS.staging.as_path())?;

    let sh = Shell::new()?;

    println!("Building cybercmd.");
    cmd!(sh, "cargo build --release").run()?;

    println!("Copying cybercmd.asi");
    sh.copy_file(
        PATHS.release.join("cybercmd.dll"),
        PATHS.staging_plugins.join("cybercmd.asi"),
    )?;

    println!("Adding Vortex install files to fomod/ directory");
    sh.copy_file(PATHS.installer.join("info.xml"), &PATHS.staging_fomod)?;
    sh.copy_file(
        PATHS.installer.join("ModuleConfig.xml"),
        &PATHS.staging_fomod,
    )?;

    println!("Adding config files (redscript)");
    for path in fs::read_dir(&PATHS.config)? {
        sh.copy_file(path?.path(), &PATHS.staging_config)?;
    }

    println!("Creating: {}", &PATHS.release.join("cybercmd.zip").quote());
    zip_files(&PATHS.staging, PATHS.release.join("cybercmd.zip"))?;

    println!("Downloading global.ini");
    download_file(PATHS.global_ini_url, global_ini)?;
    println!("Downloading version.dll");
    download_file(PATHS.version_dll_url, version_dll)?;

    println!(
        "Creating: {}",
        &PATHS.release.join("cybercmd-standalone.zip").quote()
    );
    zip_files(
        &PATHS.staging,
        PATHS.release.join("cybercmd-standalone.zip"),
    )?;

    println!();
    println!("Done!");

    Ok(())
}
