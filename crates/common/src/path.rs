use std::{fs::create_dir_all, io};

use normpath::BasePathBuf;
pub use normpath::{error::*, BasePath as Path, BasePathBuf as PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PathsError {
    #[error("Cannot get root directory")]
    IO {
        #[from]
        source: io::Error,
    },
    #[error("Cannot get root directory as parent of path")]
    Parent(#[from] ParentError),
    #[error("Cannot get project root, parent missing")]
    NoParent,
}

#[doc(hidden)]
pub fn _internal_make_path(path: &mut BasePathBuf) {
    *path = {
        if let Ok(normalized) = path.normalize() {
            normalized
        } else {
            path.normalize_virtually().expect("Invalid path!")
        }
    };

    // Automatically create directories to avoid errors and improve discoverability
    // Don't try to create a directory named scc.exe
    if path.extension().is_none() {
        let _ = create_dir_all(&path);
    } else if let Ok(Some(parent)) = path.parent() {
        let _ = create_dir_all(parent);
    }
}

#[macro_export]
macro_rules! make_path {
    ($first:expr, $($segments:expr),+) => {{
        let mut path = $crate::path::PathBuf::new($first).expect("Invalid base path!");
        $(path.push($segments);)*

        $crate::path::_internal_make_path(&mut path);

        path
    }}
}
