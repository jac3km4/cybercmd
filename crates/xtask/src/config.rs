use std::fs::{create_dir_all, remove_dir_all};

use common::{
    extensions::{Extensions, PathExt},
    make_path,
    path::{Error as PathError, PathBuf},
};

#[derive(Debug)]
pub struct Paths {
    pub debug: PathBuf,
    pub dist: PathBuf,
    pub release: PathBuf,
    pub root: PathBuf,
    pub staging: PathBuf,
    pub staging_bin: PathBuf,
    pub staging_config: PathBuf,
    pub staging_plugins: PathBuf,
    pub examples: PathBuf,
}

#[derive(Debug)]
pub struct Urls<'a> {
    pub version_dll: &'a str,
    pub global_ini: &'a str,
}

#[derive(Debug)]
pub struct Config<'a> {
    pub paths: Paths,
    pub urls: Urls<'a>,
    pub cargo_cmd: String,
}

impl Paths {
    fn new() -> Paths {
        let root = Self::project_root().unwrap();
        let target_platform = Self::target_platform_path().unwrap();
        let staging = make_path!(&root, "target", "staging");
        let staging_bin = make_path!(&staging, "bin", "x64");

        Paths {
            dist: make_path!(&root, "target", "dist"),
            release: make_path!(&target_platform, "release"),
            debug: make_path!(&target_platform, "debug"),
            staging_config: make_path!(&staging, "r6", "config", "cybercmd"),
            staging_plugins: make_path!(&staging_bin, "plugins"),
            examples: make_path!(&root, "examples"),

            // Order matters, items referenced in peers must be at the end
            staging_bin,
            staging,
            root,
        }
    }

    pub fn clean_staging(&self) -> anyhow::Result<()> {
        println!("Removing: {:?}", &self.staging);
        remove_dir_all(&self.staging)?;
        create_dir_all(&self.staging)?;
        create_dir_all(&self.staging_bin)?;
        create_dir_all(&self.staging_plugins)?;
        create_dir_all(&self.staging_config)?;
        Ok(())
    }

    pub fn clean_dist(&self) -> anyhow::Result<()> {
        println!("Removing: {:?}", &self.dist);
        remove_dir_all(&self.dist)?;
        create_dir_all(&self.dist)?;
        Ok(())
    }

    fn project_root() -> Result<PathBuf, PathError> {
        let manifest_dir = PathBuf::new(env!("CARGO_MANIFEST_DIR"))?;
        let exe_path = std::env::current_exe()?;
        let root = manifest_dir.common_root(exe_path)?;

        Ok(root)
    }

    // This will be the /target directory, or something like /target/x86_64-pc-windows-msvc depending
    // on cargo's invocation and options
    fn target_platform_path() -> Result<PathBuf, PathError> {
        let root = std::env::current_exe()?
            .normalize()?
            .ancestors()
            .nth(2)
            .ok_or(PathError::NoParent)?
            .normalize_virtually()?;

        Ok(root)
    }
}

impl<'a> Config<'a> {
    pub fn new() -> Config<'a> {
        Config {
            paths: Paths::new(),
            urls: Urls::new(),
            cargo_cmd: std::env::var("CARGO").unwrap_or("cargo".to_string()),
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
