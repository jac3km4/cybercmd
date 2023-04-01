use std::collections::HashMap;

use log::debug;
use microtemplate::render;

use super::Paths;
use crate::AppContext;

#[derive(Clone, Debug)]
pub struct ArgumentContext(HashMap<String, String>);

impl ArgumentContext {
    #[must_use]
    pub fn new(paths: &Paths) -> Self {
        Self(
            [(
                "game_dir".to_string(),
                paths.game.as_os_str().to_string_lossy().to_string(),
            )]
            .into(),
        )
    }

    #[must_use]
    pub fn from(context: &AppContext, hash_map: &HashMap<String, String>) -> Self {
        let mut new_context = Self::new(&context.paths);
        new_context.0.extend(hash_map.iter().map(|(key, val)| {
            (
                key.to_string(),
                render(val, context.argument_context.clone()),
            )
        }));
        debug!("Created new ArgumentContext: {:?}", new_context);
        new_context
    }
}

impl microtemplate::Context for ArgumentContext {
    fn get_field(&self, field_name: &str) -> &str {
        self.0.get(field_name).map(AsRef::as_ref).unwrap_or("")
    }
}
