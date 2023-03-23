use common::{
    extensions::*,
    make_path,
    path::{PathBuf, PathsError},
};
use log::info;

use super::{setup_logging, ArgumentContext, GameConfigList};

pub struct Paths {
    pub game: PathBuf,
    pub logs: PathBuf,
    pub configs: PathBuf,
    pub tools: PathBuf,
    pub scc: PathBuf,
}

impl Default for Paths {
    fn default() -> Self {
        Self::new()
    }
}

pub struct AppContext {
    pub paths: Paths,
    pub game_configs: GameConfigList,
    pub argument_context: ArgumentContext,
}

impl Paths {
    fn new() -> Paths {
        let game = Paths::get_game_path().unwrap();

        Paths {
            logs: make_path!(&game, "r6", "logs"),
            configs: make_path!(&game, "r6", "config", "cybercmd"),
            tools: make_path!(&game, "tools", "cybercmd"),
            scc: make_path!(&game, "engine", "tools", "scc.exe"),
            game,
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
}

impl AppContext {
    pub fn new() -> anyhow::Result<AppContext> {
        let paths = Paths::new();
        Self::setup_logging(&paths)?;
        info!("== Loading Cybercmd ==");

        let app_context = AppContext {
            game_configs: GameConfigList::new(&paths)?,
            argument_context: ArgumentContext::new(&paths),
            paths,
        };
        Ok(app_context)
    }

    pub fn setup_logging(paths: &Paths) -> anyhow::Result<()> {
        setup_logging(paths)
    }
}
