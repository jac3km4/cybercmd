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
#[serde(untagged)]
pub enum Task {
    V1 {
        command: String,
        path: String,
        custom_cache_dir: String,
        #[serde(default = "default_as_false")]
        terminate_on_errors: bool,
    },
    V2 {
        command: String,
        #[serde(default = "default_as_false")]
        terminate_on_errors: bool,
        #[serde(default = "default_as_true")]
        no_window: bool,
        #[serde(default)]
        template_args: Vec<String>,
        #[serde(default)]
        substitutions: HashMap<String, String>,
    },
}
