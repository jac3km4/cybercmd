use std::ffi::OsString;

use normpath::PathExt;

use crate::path::{Path, PathBuf, PathsError};

pub trait BasePathExt: private::Sealed {
    fn ancestors(&self) -> BasePathExtAncestors<'_>;
    fn common_root<P: AsRef<std::path::Path>>(&self, sibling: P) -> Result<PathBuf, PathsError>;
    fn relative_to<P: AsRef<std::path::Path>>(&self, sibling: P) -> Result<PathBuf, PathsError>;
}

impl BasePathExt for Path {
    fn ancestors(&self) -> BasePathExtAncestors<'_> {
        BasePathExtAncestors { next: Some(self) }
    }

    fn common_root<P: AsRef<std::path::Path>>(&self, sibling: P) -> Result<PathBuf, PathsError> {
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
            return Err(PathsError::NoCommonRoot);
        }
        Ok(common_root)
    }

    fn relative_to<P: AsRef<std::path::Path>>(&self, sibling: P) -> Result<PathBuf, PathsError> {
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
            return Err(PathsError::NoCommonRoot);
        }
        Ok(relative_branch)
    }
}

// Path ancestors code adapted from rust std library. MIT or Apache-2.0 licensed.
// See https://github.com/rust-lang/rust/blob/master/library/std/src/path.rs
#[derive(Copy, Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct BasePathExtAncestors<'a> {
    next: Option<&'a Path>,
}

impl<'a> Iterator for BasePathExtAncestors<'a> {
    type Item = &'a Path;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next;
        self.next = next.and_then(Path::parent_unchecked);
        next
    }
}

impl std::iter::FusedIterator for BasePathExtAncestors<'_> {}

mod private {
    pub trait Sealed {}
    impl Sealed for normpath::BasePath {}
}
