#[cfg(feature = "path")]
pub use base_path::Extensions;
#[cfg(feature = "path")]
pub use normpath::PathExt;

#[cfg(feature = "path")]
mod base_path;
