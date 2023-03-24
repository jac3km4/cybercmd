use std::{matches, path::Path};

pub fn is_valid_exe() -> bool {
    let exe = std::env::current_exe();
    let stem = exe.as_deref().ok().and_then(Path::file_stem);
    matches!(stem, Some(exe) if exe.eq_ignore_ascii_case("Cyberpunk2077") || exe.eq_ignore_ascii_case("test"))
}
