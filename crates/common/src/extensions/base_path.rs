use std::ffi::OsString;

use normpath::PathExt;

use crate::path::{Error as PathError, Path, PathBuf};

pub trait Extensions: private::Sealed {
    fn ancestors(&self) -> AncestorsExtension<'_>;
    /// # Errors
    /// Returns `PathErrors`
    fn common_root(&self, sibling: impl AsRef<std::path::Path>) -> Result<PathBuf, PathError>;
    /// # Errors
    /// Returns `PathErrors`
    fn relative_to(&self, sibling: impl AsRef<std::path::Path>) -> Result<PathBuf, PathError>;
}

impl Extensions for Path {
    fn ancestors(&self) -> AncestorsExtension<'_> {
        AncestorsExtension { next: Some(self) }
    }

    fn common_root(&self, sibling: impl AsRef<std::path::Path>) -> Result<PathBuf, PathError> {
        let me = self.normalize()?;
        let me = me.components();
        let sibling = sibling.as_ref().normalize()?;
        let sibling = sibling.components();

        let mut common_root = PathBuf::new(OsString::new())?;

        for match_components in me.zip(sibling) {
            if match_components.0 != match_components.1 {
                break;
            }
            common_root.push(match_components.0.as_os_str());
        }
        if common_root.as_os_str().is_empty() {
            return Err(PathError::NoCommonRoot);
        }
        Ok(common_root)
    }

    fn relative_to(&self, sibling: impl AsRef<std::path::Path>) -> Result<PathBuf, PathError> {
        let me = self.normalize()?;
        let me = me.components();
        let sibling = sibling.as_ref().normalize()?;
        let sibling = sibling.components();

        let mut relative_branch = PathBuf::new(OsString::new())?;

        for match_components in me.zip(sibling) {
            if match_components.0 != match_components.1 {
                break;
            }
            relative_branch.push(match_components.0.as_os_str());
        }
        if relative_branch.as_os_str().is_empty() {
            return Err(PathError::NoCommonRoot);
        }
        Ok(relative_branch)
    }
}

// Path ancestors code adapted from rust std library. MIT or Apache-2.0 licensed.
// See https://github.com/rust-lang/rust/blob/master/library/std/src/path.rs
#[derive(Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct AncestorsExtension<'a> {
    next: Option<&'a Path>,
}

impl<'a> Iterator for AncestorsExtension<'a> {
    type Item = &'a Path;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next;
        self.next = next.and_then(Path::parent_unchecked);
        next
    }
}

impl std::iter::FusedIterator for AncestorsExtension<'_> {}

mod private {
    pub trait Sealed {}
    impl Sealed for normpath::BasePath {}
}
