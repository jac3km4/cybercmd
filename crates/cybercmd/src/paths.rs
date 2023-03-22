use common::{
    extensions::*,
    make_path,
    path::{PathBuf, PathsError},
};
use once_cell::sync::Lazy;

pub struct Paths {
    pub game: PathBuf,
    pub logs: PathBuf,
    pub configs: PathBuf,
    pub tools: PathBuf,
    pub scc: PathBuf,
    pub game_str: String,
}

pub static PATHS: Lazy<Paths> = Lazy::new(get_paths);

fn get_paths() -> Paths {
    let game = get_game_path().unwrap();
    let logs = make_path!(&game, "r6", "logs");
    let configs = make_path!(&game, "r6", "config", "cybercmd");
    let tools = make_path!(&game, "tools", "cybercmd");
    let scc = make_path!(&game, "engine", "tools", "scc.exe");
    let game_str = game.as_os_str().to_string_lossy().to_string();

    Paths {
        game,
        logs,
        configs,
        tools,
        scc,
        game_str,
    }
}

fn get_game_path() -> Result<PathBuf, PathsError> {
    #[rustfmt::skip]
    let game_path = std::env::current_exe()?
        .normalize()?
        .ancestors().nth(3).ok_or(PathsError::NoParent)?
        .normalize_virtually()?;

    Ok(game_path)
}
