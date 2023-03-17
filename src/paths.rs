use common::{make_path, PathBuf, PathExt};
use once_cell::sync::Lazy;

// // TODO [AndASM]: Learn how to write idiomatic documentation comments
/*
   In this module: Lazily initialize a singleton with all our paths pre-computed.
*/

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
    // // TODO [AndASM]: Can this all be done in the Paths {} literal without cloning game?
    let game = get_game_path();
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

fn get_game_path() -> PathBuf {
    let exe = std::env::current_exe()
        .expect("Cybercmd cannot identify running exe!")
        .normalize()
        .expect("Cybercmd cannot find the path of the running exe!")
        .normalize()
        .expect("Cybercmd cannot parse the path of the running exe!");
    exe.parent_unchecked()
        .unwrap()
        .parent_unchecked()
        .unwrap()
        .parent_unchecked()
        .unwrap()
        .normalize_virtually()
        .unwrap()
}
