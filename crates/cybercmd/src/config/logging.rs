use std::path::Path;

use anyhow::Result;
use log::LevelFilter;
use log4rs::{
    append::rolling_file::{
        policy::compound::{
            roll::fixed_window::FixedWindowRoller, trigger::size::SizeTrigger, CompoundPolicy,
        },
        RollingFileAppender,
    },
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    Config,
};

use crate::config::app_context::Paths;

pub(super) fn setup_logging(paths: &Paths) -> Result<()> {
    let window_roller = FixedWindowRoller::builder()
        .build(
            Path::join(Path::new(&paths.logs), "cybercmd{}.log")
                .to_string_lossy()
                .as_ref(),
            3,
        )
        .unwrap();
    let size_trigger = SizeTrigger::new(1024 * 50);
    let policy = CompoundPolicy::new(Box::new(size_trigger), Box::new(window_roller));

    let logfile = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "[{d(%Y-%m-%d %H:%M:%S)}] {l:<5.5}: {m:}\n",
        )))
        .build(
            Path::join(Path::new(&paths.logs), "cybercmd.log"),
            Box::new(policy),
        )?;

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(
            Root::builder()
                .appender("logfile")
                .build(LevelFilter::Debug),
        )?;

    log4rs::init_config(config)?;

    Ok(())
}
