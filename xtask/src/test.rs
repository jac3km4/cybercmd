use std::{env::set_current_dir, fs};

use anyhow::Result;
use common::download_file;
use xshell::{cmd, Shell};

use crate::common::config::PATHS;

pub fn test() -> Result<()> {
    set_current_dir(PATHS.debug.as_path())?;

    let sh = Shell::new()?;

    println!("Build cybercmd.dll (.asi)");
    cmd!(sh, "cargo build").run()?;
    println!("Build test.exe");
    cmd!(sh, "cargo build --package test").run()?;

    println!("Copying cybercmd.asi");
    fs::create_dir_all(PATHS.debug.join("plugins"))?;

    sh.copy_file(
        PATHS.debug.join("cybercmd.dll"),
        PATHS.debug.join("plugins").join("cybercmd.asi"),
    )?;

    println!("Downloading global.ini");
    download_file(PATHS.global_ini_url, PATHS.debug.join("global.ini"))?;
    println!("Downloading version.dll");
    download_file(PATHS.version_dll_url, PATHS.debug.join("version.dll"))?;

    println!("Running test.exe");
    cmd!(sh, "./test.exe").run()?;

    println!();
    println!("Done!");

    Ok(())
}
