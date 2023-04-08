use std::{ffi::OsStr, fs};

use common::extensions::PathExt;
use uniquote::Quote;

use super::{app_context::Paths, GameConfig};

pub struct GameConfigList(Vec<GameConfig>);

impl<T> core::ops::Index<T> for GameConfigList
where
    Vec<GameConfig>: core::ops::Index<T>,
{
    type Output = <Vec<GameConfig> as core::ops::Index<T>>::Output;

    #[inline]
    fn index(&self, idx: T) -> &Self::Output {
        self.0.index(idx)
    }
}

impl IntoIterator for GameConfigList {
    type IntoIter = <Vec<GameConfig> as IntoIterator>::IntoIter;
    type Item = <Vec<GameConfig> as IntoIterator>::Item;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
       self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a GameConfigList {
    type IntoIter = <&'a Vec<GameConfig> as IntoIterator>::IntoIter;
    type Item = <&'a Vec<GameConfig> as IntoIterator>::Item;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

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
                            Some(val) => format!("{val:?}"),
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
