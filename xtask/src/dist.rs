use std::{env::set_current_dir, fs};

use anyhow::Result;
use common::{download_file, zip_files};
use xshell::{cmd, Shell};

use crate::common::config::PATHS;

pub fn dist() -> Result<()> {
    let global_ini = PATHS.staging_bin.join("global.ini");
    let version_dll = PATHS.staging_bin.join("version.dll");

    println!("Cleanup staging");
    fs::remove_dir_all(&PATHS.staging)?;
    fs::create_dir_all(&PATHS.staging)?;
    fs::create_dir_all(&PATHS.staging_bin)?;
    fs::create_dir_all(&PATHS.staging_plugins)?;

    set_current_dir(PATHS.staging.as_path())?;

    let sh = Shell::new()?;

    cmd!(sh, "cargo build --release").run()?;

    println!("Copying cybercmd.asi");
    sh.copy_file(
        PATHS.release.join("cybercmd.dll"),
        PATHS.staging_plugins.join("cybercmd.asi"),
    )?;

    println!(
        "Creating: {}",
        &PATHS
            .release
            .join("cybercmd.zip")
            .as_os_str()
            .to_string_lossy()
    );
    zip_files(&PATHS.staging, PATHS.release.join("cybercmd.zip"))?;

    println!("Downloading global.ini");
    download_file(PATHS.global_ini_url, global_ini)?;
    println!("Downloading version.dll");
    download_file(PATHS.version_dll_url, version_dll)?;

    println!(
        "Creating: {}",
        &PATHS
            .release
            .join("cybercmd-standalone.zip")
            .as_os_str()
            .to_string_lossy()
    );
    zip_files(
        &PATHS.staging,
        PATHS.release.join("cybercmd-standalone.zip"),
    )?;

    println!();
    println!("Done!");

    Ok(())
}
