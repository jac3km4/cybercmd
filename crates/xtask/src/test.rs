use std::{env::set_current_dir, fs};

use anyhow::Result;
use xshell::{cmd, Shell};

use common::file::download_file;

use crate::config::CONFIG;

pub fn test() -> Result<()> {
    println!("Cleanup staging");
    CONFIG.paths.clean_staging()?;

    set_current_dir(&CONFIG.paths.staging_bin)?;
    let sh = Shell::new()?;

    println!("Build test.exe and cybercmd.dll (.asi)");
    cmd!(sh, "cargo build --package cybercmd --package test").run()?;

    println!("Copying cybercmd.asi and test.exe");
    sh.copy_file(
        CONFIG.paths.debug.join("cybercmd.dll"),
        CONFIG.paths.staging_plugins.join("cybercmd.asi"),
    )?;
    sh.copy_file(
        CONFIG.paths.debug.join("test.exe"),
        &CONFIG.paths.staging_bin,
    )?;

    println!("Downloading global.ini");
    download_file(
        CONFIG.urls.global_ini,
        CONFIG.paths.staging_bin.join("global.ini"),
    )?;
    println!("Downloading version.dll");
    download_file(
        CONFIG.urls.version_dll,
        CONFIG.paths.staging_bin.join("version.dll"),
    )?;

    println!("Running test.exe");
    cmd!(sh, "./test.exe").run()?;

    println!();
    println!("Done!");

    Ok(())
}
