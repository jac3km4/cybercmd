#![feature(macro_metavar_expr)]

use std::env;

use crate::{config::Config, dist::dist, install::install, scratch::scratch, test::test};

mod config;
mod dist;
mod install;
mod scratch;
mod stage;
mod test;

#[macro_export]
macro_rules! make_cli {
    ($first:expr, $($segments:expr),+) => {};
}

make_cli!(dist, install, scratch);

fn main() {
    try_main().expect("Unhandled error");
}

fn try_main() -> anyhow::Result<()> {
    let config = Config::new();

    let command = env::args().nth(1);
    let remain = env::args().skip(2).collect::<Vec<String>>();
    let game_dir = remain.join(" ");
    match command.as_deref() {
        Some("scratch") => scratch(&config)?,
        Some("dist") => dist(&config)?,
        Some("install") => install(&config, game_dir)?,
        Some("test") => test(&config)?,
        _ => panic!("Unknown command"),
    }

    Ok(())
}
