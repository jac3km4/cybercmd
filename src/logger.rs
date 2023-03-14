use std::fs::File;
use std::io::Write;
use std::time::SystemTime;

use anyhow::{Error, Result};
use chrono::prelude::*;

use crate::paths;

pub struct Logger {
    file: Option<File>,
    error: Option<Error>,
    level: i8,
}

impl Logger {
    #[cfg_attr(feature = "cargo-clippy", allow(clippy::new_without_default))]
    pub fn new() -> Logger {
        let int_level = std::env::var("CYBERCMD_DEBUG")
            .unwrap_or("1".to_string())
            .parse::<i8>()
            .unwrap_or(1);
        let log_file_path = match paths::get_log_file_path() {
            Ok(path) => path,
            Err(e) => return Logger::from_error(e),
        };

        let is_modified_before_today = DateTime::<Local>::from(
            std::fs::metadata(&log_file_path)
                .ok()
                .and_then(|md| md.modified().ok())
                .unwrap_or(SystemTime::UNIX_EPOCH),
        )
        .date_naive()
            < Local::now().date_naive();

        let file = match File::options()
            .create(true)
            .write(true)
            .truncate(is_modified_before_today)
            .append(!is_modified_before_today)
            .open(log_file_path)
        {
            Ok(file) => file,
            Err(e) => return Logger::from_error(Error::from(e)),
        };

        let mut this = Logger {
            file: Some(file),
            error: None,
            level: int_level,
        };

        this.debug("Starting cybercmd");

        this
    }

    pub fn from_error(error: Error) -> Logger {
        Logger {
            file: None,
            error: Some(error),
            level: 0,
        }
    }

    pub fn log(&mut self, level: i8, message: String) -> Result<()> {
        if let Some(file) = &mut self.file {
            if self.level >= level {
                return Ok(writeln!(
                    file,
                    "[{}]{}",
                    Local::now().round_subsecs(3),
                    message
                )?);
            }
        }

        match &self.error {
            Some(error) => Err(Error::msg(error.to_string())),
            None => Err(Error::msg("Unknown error preventing log creation")),
        }
    }

    pub fn debug<S: Into<String>>(&mut self, message: S) {
        let _ = self.log(4, format!("[DEBUG] {}", message.into()));
    }

    pub fn info<S: Into<String>>(&mut self, message: S) {
        let _ = self.log(3, format!("[INFO] {}", message.into()));
    }

    pub fn warn<S: Into<String>>(&mut self, message: S) {
        let _ = self.log(2, format!("[WARN] {}", message.into()));
    }

    pub fn error<S: Into<String>>(&mut self, message: S) {
        let _ = self.log(1, format!("[ERROR] {}", message.into()));
    }
}
