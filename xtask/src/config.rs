use common::{make_path, ParentError, PathBuf};
use normpath::BasePathBuf;
use once_cell::sync::Lazy;
use common::path::PathsError;

pub static PATHS: Lazy<Paths> = Lazy::new(Paths::new);

pub struct Paths {
    pub root: PathBuf,
    pub dist: PathBuf,
    pub staging: PathBuf,
    pub release: PathBuf,
    pub debug: PathBuf,
    pub staging_bin: PathBuf,
    pub staging_plugins: PathBuf,
    pub version_dll_url: &'static str,
    pub global_ini_url: &'static str,
}

impl Paths {
    fn new() -> Paths {
        let root = project_root().unwrap();
        let dist = make_path!(&root, "target", "dist");
        let staging = make_path!(&root, "target", "staging");
        let release = make_path!(&root, "target", "release");
        let debug = make_path!(&root, "target", "debug");
        let staging_bin = make_path!(&staging, "bin", "x64");
        let staging_plugins = make_path!(&staging_bin, "plugins");

        Paths {
            root,
            dist,
            staging,
            release,
            debug,
            staging_bin,
            staging_plugins,
            version_dll_url: "https://raw.githubusercontent.com/yamashi/CyberEngineTweaks/master/vendor/asiloader/version.dll",
            global_ini_url: "https://raw.githubusercontent.com/yamashi/CyberEngineTweaks/master/vendor/asiloader/global.ini",
        }
    }
}

fn project_root() -> Result<PathBuf, PathsError> {
    #[rustfmt::skip]
    let root = BasePathBuf::new(env!("CARGO_MANIFEST_DIR"))?
        .normalize()?
        .parent()?.ok_or(PathsError::NoParent)?
        .normalize_virtually()?;

    Ok(root)
}
