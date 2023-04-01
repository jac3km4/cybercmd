use std::collections::HashMap;

use serde::Deserialize;

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
