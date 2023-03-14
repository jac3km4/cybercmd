use std::path::PathBuf;

pub fn get_game_path() -> anyhow::Result<PathBuf> {
    let exe = std::env::current_exe()?;
    let path = exe.parent().unwrap().parent().unwrap().parent().unwrap();
    Ok(path.to_path_buf())
}

pub fn get_log_file_path() -> anyhow::Result<PathBuf> {
    let game_path = match get_game_path() {
        Ok(path) => path,
        Err(e) => return Err(e),
    };
    Ok(game_path.join("r6").join("logs").join("cybercmd.log"))
}
