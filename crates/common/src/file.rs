use std::{fs::File, path::Path};

use reqwest::blocking as reqwest;
use zip::CompressionMethod;
use zip_extensions::ZipWriterExtensions;

pub fn download_file<S: AsRef<str>, P: AsRef<Path>>(source: S, dest: P) -> anyhow::Result<()> {
    let mut response = reqwest::get(source.as_ref())?;
    let mut file = File::create(dest.as_ref())?;
    let _ = &response.copy_to(&mut file)?;
    Ok(())
}

pub fn zip_files<P1: AsRef<Path>, P2: AsRef<Path>>(
    source: P1,
    destination: P2,
) -> anyhow::Result<()> {
    let mut dest_file = File::create(destination.as_ref())?;
    let mut zip = zip::ZipWriter::new(&mut dest_file);
    let options = zip::write::FileOptions::default()
        .compression_method(CompressionMethod::Deflated)
        .compression_level(Some(9));

    zip.create_from_directory_with_options(source, options)?;

    Ok(())
}
