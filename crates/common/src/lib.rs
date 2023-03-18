pub use normpath::BasePathBuf as PathBuf;
pub use normpath::PathExt;
pub use normpath::error::*;
use std::path::Path;
use anyhow::Result;
use std::fs::File;
use reqwest::blocking as reqwest;
use zip::CompressionMethod;
use zip_extensions::write::ZipWriterExtensions;

#[macro_export]
macro_rules! make_path {
    ($first:expr, $($segments:expr),+) => {{
        let mut path = common::PathBuf::new($first).expect("Invalid base path!");
        $(path.push($segments);)*

        path = {
            if let Ok(normalized) = path.normalize() {
                normalized
            } else {
                path.normalize_virtually().expect("Invalid path!")
            }
        };

        // Automatically create directories to avoid errors and improve discoverability
        // Don't try to create a directory named scc.exe
        if path.extension().is_none() {
            let _ = std::fs::create_dir_all(&path);
        } else {
            if let Ok(base_parent) = path.parent() {
                if let Some(parent) = base_parent {
                    let _ = std::fs::create_dir_all(&parent);
                }
            }
        }

        path
    }}
}

pub fn download_file<S: Into<String>, P: AsRef<Path>>(source: S, dest: P) -> Result<()> {
    let mut response = reqwest::get(source.into())?;
    let mut file = File::create(dest.as_ref())?;
    let _ = &response.copy_to(&mut file)?;
    Ok(())
}

#[allow(deprecated)]
pub fn zip_files<P1: AsRef<Path>, P2: AsRef<Path>>(source: P1, destination: P2) -> Result<()> {
    let mut dest_file = File::create(destination.as_ref())?;
    let mut zip = zip::ZipWriter::new(&mut dest_file);
    let options = zip::write::FileOptions::default()
        .compression_method(CompressionMethod::Deflated)
        .compression_level(Some(9));

    zip.create_from_directory_with_options(source, options)?;

    Ok(())
}
