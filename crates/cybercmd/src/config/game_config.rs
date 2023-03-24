use std::{collections::HashMap, ffi::OsStr, fs};

use common::extensions::*;
use derive_more::{Index, IntoIterator};
use serde::Deserialize;
use uniquote::Quote;

use crate::config::app_context::Paths;

#[derive(Index, IntoIterator)]
pub struct GameConfigList(#[into_iterator(ref)] Vec<GameConfig>);

impl GameConfigList {
    pub fn new(paths: &Paths) -> anyhow::Result<Self> {
        let mut game_configs: Vec<GameConfig> = Vec::new();

        let path = &paths.configs;

        log::debug!("Getting configs.");

        for entry in fs::read_dir(path)? {
            let entry = entry?;

            if entry.path().extension() == Some(OsStr::new("toml")) {
                log::debug!("Loading: {}", entry.path().normalize_virtually()?.quote());
                let contents = fs::read_to_string(entry.path())?;
                match toml::from_str::<GameConfig>(&contents) {
                    Ok(mut mod_config) => {
                        mod_config.file_name = entry.path().display().to_string();
                        game_configs.push(mod_config);
                    }
                    Err(error) => log::error!(
                        "In {} ({}): {}",
                        &entry.path().normalize_virtually()?.quote(),
                        match error.span() {
                            Some(val) => format!("{:?}", val),
                            None => String::new(),
                        },
                        error.message()
                    ),
                };
            }
        }
        Ok(Self(game_configs))
    }
}

#[derive(Debug, Deserialize)]
pub struct GameConfig {
    #[serde(skip)]
    pub(crate) file_name: String,
    #[serde(default)]
    pub(crate) args: HashMap<String, String>,
    #[serde(default)]
    pub(crate) tasks: Vec<Task>,
}

fn default_as_false() -> bool {
    false
}

fn default_as_true() -> bool {
    true
}

#[derive(Debug, Deserialize, Clone)]
pub struct Task {
    pub(crate) command: String,
    #[serde(default = "default_as_false")]
    pub(crate) terminate_on_errors: bool,
    #[serde(default = "default_as_true")]
    pub(crate) no_window: bool,
    #[serde(default)]
    pub(crate) template_args: Vec<String>,
    pub(crate) substitutions: HashMap<String, String>,
}
