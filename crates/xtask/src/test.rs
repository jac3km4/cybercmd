use std::{env::set_current_dir, fs};

use anyhow::Result;
use common::file::download_file;
use xshell::{cmd, Shell};

use crate::config::Config;

pub fn test(config: &Config) -> Result<()> {
    println!("Cleanup staging");
    config.paths.clean_staging()?;

    set_current_dir(&config.paths.staging_bin)?;
    let sh = Shell::new()?;

    println!("Build test.exe and cybercmd.dll (.asi)");
    cmd!(sh, "cargo build --package cybercmd --package test").run()?;

    println!("Copying cybercmd.asi and test.exe");
    sh.copy_file(
        config.paths.debug.join("cybercmd.dll"),
        config.paths.staging_plugins.join("cybercmd.asi"),
    )?;
    sh.copy_file(
        config.paths.debug.join("test.exe"),
        &config.paths.staging_bin,
    )?;

    println!("Downloading global.ini");
    download_file(
        config.urls.global_ini,
        config.paths.staging_bin.join("global.ini"),
    )?;
    println!("Downloading version.dll");
    download_file(
        config.urls.version_dll,
        config.paths.staging_bin.join("version.dll"),
    )?;

    println!("Adding config files from examples.");
    for path in fs::read_dir(&config.paths.examples)? {
        sh.copy_file(path?.path(), &config.paths.staging_config)?;
    }

    println!("Running test.exe");
    cmd!(sh, "./test.exe").run()?;

    println!();
    println!("Done!");

    Ok(())
}
