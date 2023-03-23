use std::fs::{create_dir_all, remove_dir_all};

use common::{
    extensions::*,
    make_path,
    path::{PathBuf, PathsError},
};
use normpath::BasePathBuf;

pub struct Paths {
    pub config: PathBuf,
    pub debug: PathBuf,
    pub dist: PathBuf,
    pub installer: PathBuf,
    pub release: PathBuf,
    pub root: PathBuf,
    pub staging: PathBuf,
    pub staging_bin: PathBuf,
    pub staging_config: PathBuf,
    pub staging_plugins: PathBuf,
    pub staging_fomod: PathBuf,
    pub examples: PathBuf,
}

pub struct Urls<'a> {
    pub version_dll: &'a str,
    pub global_ini: &'a str,
}

pub struct Config<'a> {
    pub paths: Paths,
    pub urls: Urls<'a>,
}

impl Paths {
    fn new() -> Paths {
        let root = Self::project_root().unwrap();
        let staging = make_path!(&root, "target", "staging");
        let staging_bin = make_path!(&staging, "bin", "x64");

        Paths {
            dist: make_path!(&root, "target", "dist"),
            release: make_path!(&root, "target", "release"),
            debug: make_path!(&root, "target", "debug"),
            staging_config: make_path!(&staging, "r6", "config", "cybercmd"),
            staging_fomod: make_path!(&staging, "fomod"),
            staging_plugins: make_path!(&staging_bin, "plugins"),
            installer: make_path!(&root, "resources", "installer"),
            config: make_path!(&root, "resources", "config"),
            examples: make_path!(&root, "examples"),

            // Order matters, items referenced in peers must be at the end
            staging_bin,
            staging,
            root,
        }
    }

    pub fn clean_staging(&self) -> anyhow::Result<()> {
        remove_dir_all(&self.staging)?;
        create_dir_all(&self.staging)?;
        create_dir_all(&self.staging_bin)?;
        create_dir_all(&self.staging_plugins)?;
        create_dir_all(&self.staging_config)?;
        create_dir_all(&self.staging_fomod)?;
        Ok(())
    }

    fn project_root() -> Result<PathBuf, PathsError> {
        #[rustfmt::skip]
            let root = BasePathBuf::new(env!("CARGO_MANIFEST_DIR"))?
            .normalize()?
            .ancestors().nth(2).ok_or(PathsError::NoParent)?
            .normalize_virtually()?;

        Ok(root)
    }
}

impl<'a> Config<'a> {
    pub fn new() -> Config<'a> {
        Config {
            paths: Paths::new(),
            urls: Urls::new(),
        }
    }
}

impl<'a> Urls<'a> {
    fn new() -> Urls<'a> {
        Urls {
            version_dll: "https://raw.githubusercontent.com/yamashi/CyberEngineTweaks/master/vendor/asiloader/version.dll",
            global_ini: "https://raw.githubusercontent.com/yamashi/CyberEngineTweaks/master/vendor/asiloader/global.ini",
        }
    }
}
