use std::{env::set_current_dir, fs::File, path::Path};

use anyhow::Result;
use reqwest::blocking as reqwest;
use xshell::{cmd, Shell};
use zip::CompressionMethod;
use zip_extensions::write::ZipWriterExtensions;

use crate::common::config::PATHS;

pub fn dist() -> Result<()> {
    let sh = Shell::new()?;

    sh.change_dir(&PATHS.staging);
    set_current_dir(PATHS.staging.as_path())?;

    cmd!(sh, "cargo build --release").run()?;

    println!("Copying cybercmd.asi");
    sh.copy_file(
        PATHS.release.join("cybercmd.dll"),
        PATHS.staging_plugins.join("cybercmd.asi"),
    )?;

    println!("Creating: {}", &PATHS.release.join("cybercmd.zip").as_os_str().to_string_lossy());
    zip_files(&PATHS.staging, PATHS.release.join("cybercmd.zip"))?;

    println!("Downloading global.ini");
    download("https://raw.githubusercontent.com/yamashi/CyberEngineTweaks/master/vendor/asiloader/global.ini", PATHS.staging_bin.join("global.ini"))?;
    println!("Downloading version.dll");
    download("https://raw.githubusercontent.com/yamashi/CyberEngineTweaks/master/vendor/asiloader/version.dll", PATHS.staging_bin.join("version.dll"))?;

    println!("Creating: {}", &PATHS.release.join("cybercmd-standalone.zip").as_os_str().to_string_lossy());
    zip_files(
        &PATHS.staging,
        PATHS.release.join("cybercmd-standalone.zip"),
    )?;

    println!();
    println!("Done!");

    Ok(())
}

fn download<S: Into<String>, P: AsRef<Path>>(source: S, dest: P) -> Result<()> {
    let mut response = reqwest::get(source.into())?;
    let mut file = File::create(dest.as_ref())?;
    let _ = &response.copy_to(&mut file)?;
    Ok(())
}

#[allow(deprecated)]
fn zip_files<P1: AsRef<Path>, P2: AsRef<Path>>(source: P1, destination: P2) -> Result<()> {
    let mut dest_file = File::create(destination.as_ref())?;
    let mut zip = zip::ZipWriter::new(&mut dest_file);
    let options = zip::write::FileOptions::default()
        .compression_method(CompressionMethod::Deflated)
        .compression_level(Some(9));

    zip.create_from_directory_with_options(source, options)?;

    Ok(())
}
