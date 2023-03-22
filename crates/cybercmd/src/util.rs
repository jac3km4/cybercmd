use std::path::Path;

use anyhow::Result;
use log4rs::{
    append::rolling_file::{
        policy::compound::{
            CompoundPolicy, roll::fixed_window::FixedWindowRoller, trigger::size::SizeTrigger,
        },
        RollingFileAppender,
    },
    config::{Appender, Root},
    Config,
    encode::pattern::PatternEncoder,
};
use log::LevelFilter;

use crate::paths::PATHS;

pub fn setup_logging() -> Result<()> {
    let window_roller = FixedWindowRoller::builder()
        .build(
            Path::join(Path::new(&PATHS.logs), "cybercmd{}.log")
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
            Path::join(Path::new(&PATHS.logs), "cybercmd.log"),
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

pub fn is_valid_exe() -> bool {
    let exe = std::env::current_exe();
    let stem = exe.as_deref().ok().and_then(Path::file_stem);
    matches!(stem, Some(exe) if exe.eq_ignore_ascii_case("Cyberpunk2077") || exe.eq_ignore_ascii_case("test"))
}
