use std::{io, path::Path};

use normpath::PathExt as npPathExt;

use crate::PathBuf;

pub trait PathExt: private::SealedPathExt {
    fn normalize(&self) -> io::Result<PathBuf>;
    fn normalize_virtually(&self) -> io::Result<PathBuf>;
}

impl PathExt for Path {
    #[inline]
    fn normalize(&self) -> io::Result<PathBuf> {
        npPathExt::normalize(self)
    }

    #[inline]
    fn normalize_virtually(&self) -> io::Result<PathBuf> {
        npPathExt::normalize_virtually(self)
    }
}

pub trait PathBufExt: private::SealedPathBufExt {
    fn to_string(&self) -> String;
}

impl PathBufExt for PathBuf {
    fn to_string(&self) -> String {
        self.as_os_str().to_string_lossy().to_string()
    }
}

mod private {
    pub trait SealedPathExt {}
    impl SealedPathExt for std::path::Path {}

    pub trait SealedPathBufExt {}
    impl SealedPathBufExt for normpath::BasePathBuf {}
}
