use std::{collections::HashMap, ffi::OsStr, fs};

use microtemplate::render;
use serde::{Deserialize, Deserializer};
use uniquote::Quote;

use common::extensions::*;

use crate::paths::PATHS;

#[derive(Clone, Default, Debug)]
pub struct ConfigContext;

impl microtemplate::Context for ConfigContext {
    fn get_field(&self, field_name: &str) -> &str {
        match field_name {
            "game_dir" => PATHS.game_str.as_str(),
            _ => "",
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ModConfig {
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
    pub(crate) args: Vec<String>,
    #[serde(flatten, deserialize_with = "deserialize_render_map")]
    pub(crate) values: HashMap<String, String>,
}

fn deserialize_render_map<'de, D>(deserializer: D) -> Result<HashMap<String, String>, D::Error>
where
    D: Deserializer<'de>,
{
    let hash_map = HashMap::<String, String>::deserialize(deserializer)?;
    Ok(hash_map
        .into_iter()
        .map(|(key, value)| (key, render(value.as_str(), ConfigContext {})))
        .collect())
}

impl microtemplate::Context for Task {
    fn get_field(&self, field_name: &str) -> &str {
        let result = if field_name == "game_dir" {
            PATHS.game_str.as_str()
        } else if let Some(val) = self.values.get(field_name) {
            val.as_str()
        } else {
            ""
        };
        log::debug!("Task {} key {} = {}", self.command, field_name, result);

        result
    }
}

pub fn get_configs() -> anyhow::Result<Vec<ModConfig>> {
    let path = &PATHS.configs;
    let mut configs = vec![];

    log::debug!("Getting configs.");

    for entry in fs::read_dir(path)? {
        let entry = entry?;

        if entry.path().extension() == Some(OsStr::new("toml")) {
            log::debug!("Loading: {}", entry.path().normalize_virtually()?.quote());
            let contents = fs::read_to_string(entry.path())?;
            match toml::from_str::<ModConfig>(&contents) {
                Ok(config) => configs.push(config),
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
    Ok(configs)
}
