use std::{fs::File, io, path::Path};

#[cfg(feature = "zip")]
use zip::CompressionMethod;
#[cfg(feature = "zip")]
use zip_extensions::ZipWriterExtensions;

#[cfg(feature = "download")]
/// # Errors
/// Returns `anyhow::Error` wrapping a `native_tls::Error`, `ureq::Error`, or `std::io::Error`
pub fn download(source: impl AsRef<str>, dest: impl AsRef<Path>) -> anyhow::Result<()> {
    let agent = ureq::Agent::config_builder()
        .tls_config(
            ureq::tls::TlsConfig::builder()
                .provider(ureq::tls::TlsProvider::NativeTls)
                .root_certs(ureq::tls::RootCerts::PlatformVerifier)
                .build(),
        )
        .build()
        .new_agent();

    let response = agent.get(source.as_ref()).call()?;
    let mut file = File::create(dest.as_ref())?;
    let mut reader = response.into_body().into_reader();
    io::copy(&mut reader, &mut file)?;
    Ok(())
}

#[cfg(feature = "zip")]
/// # Errors
/// Returns `anyhow::Error` wrapping a `zip::ZipError`, or `std::io::Error`
pub fn zip_files(source: impl AsRef<Path>, destination: impl AsRef<Path>) -> anyhow::Result<()> {
    let mut dest_file = File::create(destination.as_ref())?;
    let zip = zip::ZipWriter::new(&mut dest_file);
    let options = zip::write::SimpleFileOptions::default()
        .compression_method(CompressionMethod::Deflated)
        .compression_level(Some(9));

    zip.create_from_directory_with_options(&source.as_ref().to_owned(), |_| options)?;

    Ok(())
}
