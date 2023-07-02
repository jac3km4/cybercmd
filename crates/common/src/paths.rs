use std::path::Path;

use crate::{
    extensions::{Extensions, PathExt},
    make_path,
    path::{Error as PathError, PathBuf},
};

pub struct Paths {
    root: PathBuf,
}

impl Paths {
    /// # Errors
    /// Returns `PathError`
    pub fn new() -> Result<Self, PathError> {
        let root = std::env::current_exe()?
            .normalize()?
            .ancestors()
            .nth(3)
            .ok_or(PathError::NoParent)?
            .normalize_virtually()?;

        Ok(Self { root })
    }

    #[must_use]
    pub fn game_dir(&self) -> impl AsRef<Path> + '_ {
        &self.root
    }

    #[must_use]
    pub fn tools_dir(&self) -> impl AsRef<Path> + '_ {
        make_path!(&self.root, "tools", "cybercmd")
    }

    #[must_use]
    pub fn scc_exe(&self) -> impl AsRef<Path> + '_ {
        make_path!(&self.root, "engine", "tools", "scc.exe")
    }

    #[must_use]
    pub fn cybercmd_config_dir(&self) -> impl AsRef<Path> + '_ {
        make_path!(&self.root, "r6", "config", "cybercmd")
    }

    #[must_use]
    pub fn log_dir(&self) -> impl AsRef<Path> + '_ {
        make_path!(&self.root, "r6", "logs")
    }
}
