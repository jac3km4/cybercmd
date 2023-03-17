use common::{make_path, PathBuf};
use normpath::BasePathBuf;
use once_cell::sync::Lazy;

pub static PATHS: Lazy<Paths> = Lazy::new(Paths::new);

pub struct Paths {
    pub root: PathBuf,
    pub dist: PathBuf,
    pub staging: PathBuf,
    pub release: PathBuf,
    pub staging_bin: PathBuf,
    pub staging_plugins: PathBuf,
}

impl Paths {
    fn new() -> Paths {
        let root = project_root();
        let dist = make_path!(&root, "target", "dist");
        let staging = make_path!(&root, "target", "staging");
        let release = make_path!(&root, "target", "release");
        let staging_bin = make_path!(&staging, "bin", "x64");
        let staging_plugins = make_path!(&staging_bin, "plugins");

        Paths {
            root,
            dist,
            staging,
            release,
            staging_bin,
            staging_plugins,
        }
    }
}

fn project_root() -> PathBuf {
    let root = BasePathBuf::new(env!("CARGO_MANIFEST_DIR"))
        .expect("Cannot read project dir from CARGO_MANIFEST_DIR")
        .normalize()
        .expect("Cannot find project dir")
        .normalize()
        .expect("Cannot parse project dir");
    root.parent_unchecked()
        .unwrap()
        .parent_unchecked()
        .unwrap()
        .normalize_virtually()
        .unwrap()
}
