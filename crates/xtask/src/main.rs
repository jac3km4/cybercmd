#![feature(macro_metavar_expr)]

use clap::{Parser, Subcommand};

use crate::{config::Config, dist::dist, install::install, scratch::scratch, test::test};

mod config;
mod dist;
mod install;
mod scratch;
mod test;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Scratch,
    Dist,
    Install { game_dir: String },
    Test,
}

fn main() {
    try_main().expect("Unhandled error")
}

fn try_main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let config = Config::new();

    match &cli.command {
        Commands::Scratch => scratch(&config)?,
        Commands::Dist => dist(&config)?,
        Commands::Install { game_dir } => install(&config, game_dir)?,
        Commands::Test => test(&config)?,
    }

    Ok(())
}
