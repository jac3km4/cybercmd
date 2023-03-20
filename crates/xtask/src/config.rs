use common::{make_path, path::PathsError, PathBuf};
use normpath::BasePathBuf;
use once_cell::sync::Lazy;

pub static PATHS: Lazy<Paths> = Lazy::new(Paths::new);

pub struct Paths {
    pub config: PathBuf,
    pub debug: PathBuf,
    pub dist: PathBuf,
    pub global_ini_url: &'static str,
    pub installer: PathBuf,
    pub release: PathBuf,
    pub root: PathBuf,
    pub staging: PathBuf,
    pub staging_bin: PathBuf,
    pub staging_config: PathBuf,
    pub staging_plugins: PathBuf,
    pub staging_fomod: PathBuf,
    pub version_dll_url: &'static str,
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
        let installer = make_path!(&root, "resources", "installer");
        let config = make_path!(&root, "resources", "config");
        let staging_fomod = make_path!(&staging, "fomod");
        let staging_config = make_path!(&staging, "r6", "config", "cybercmd");

        Paths {
            root,
            dist,
            staging,
            release,
            debug,
            staging_bin,
            staging_config,
            staging_fomod,
            staging_plugins,
            version_dll_url: "https://raw.githubusercontent.com/yamashi/CyberEngineTweaks/master/vendor/asiloader/version.dll",
            global_ini_url: "https://raw.githubusercontent.com/yamashi/CyberEngineTweaks/master/vendor/asiloader/global.ini",
            installer,
            config,
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
