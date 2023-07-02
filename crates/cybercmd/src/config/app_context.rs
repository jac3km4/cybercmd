use common::paths::Paths;
use log::info;

use super::{ArgumentContext, GameConfigList};

pub struct AppContext {
    pub paths: Paths,
    pub game_configs: GameConfigList,
    pub argument_context: ArgumentContext,
}

impl AppContext {
    /// # Errors
    /// Returns `anyhow::Error` aggregating many error types.
    pub fn new() -> anyhow::Result<AppContext> {
        let paths = Paths::new()?;
        info!("Loading Cybercmd");

        let app_context = AppContext {
            game_configs: GameConfigList::new(&paths)?,
            argument_context: ArgumentContext::new(&paths),
            paths,
        };
        Ok(app_context)
    }
}
