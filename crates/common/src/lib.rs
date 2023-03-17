pub use normpath::BasePathBuf as PathBuf;
pub use normpath::PathExt;

#[macro_export]
macro_rules! make_path {
    ($first:expr, $($segments:expr),+) => {{
        let mut path = common::PathBuf::new($first).expect("Invalid base path!");
        $(path.push($segments);)*

        path = {
            if let Ok(normalized) = path.normalize() {
                normalized
            } else {
                path.normalize_virtually().expect("Invalid path!")
            }
        };

        // Automatically create directories to avoid errors and improve discoverability
        // Don't try to create a directory named scc.exe
        if path.extension().is_none() {
            let _ = std::fs::create_dir_all(&path);
        } else {
            if let Ok(base_parent) = path.parent() {
                if let Some(parent) = base_parent {
                    let _ = std::fs::create_dir_all(&parent);
                }
            }
        }

        path
    }}
}
