use std::{fs::File, io::copy, path::Path, sync::Arc};

#[cfg(feature = "zip")]
use zip::CompressionMethod;
#[cfg(feature = "zip")]
use zip_extensions::ZipWriterExtensions;

#[cfg(feature = "download")]
/// # Errors
/// Returns `anyhow::Error` wrapping a `native_tls::Error`, `ureq::Error`, or `std::io::Error`
pub fn download(source: impl AsRef<str>, dest: impl AsRef<Path>) -> anyhow::Result<()> {
    // Use native-tls (Windows' tls)
    let agent = ureq::AgentBuilder::new()
        .tls_connector(Arc::new(native_tls::TlsConnector::new()?))
        .build();

    let response = agent.get(source.as_ref()).call()?;
    let mut file = File::create(dest.as_ref())?;
    let mut reader = response.into_reader();
    copy(&mut reader, &mut file)?;
    Ok(())
}

#[cfg(feature = "zip")]
/// # Errors
/// Returns `anyhow::Error` wrapping a `zip::ZipError`, or `std::io::Error`
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
