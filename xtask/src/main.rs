#![feature(macro_metavar_expr)]

use clap::{Parser, Subcommand};

use crate::{dist::dist, scratch::scratch};
use crate::install::install;
use crate::test::test;

mod common;
mod dist;
mod scratch;
mod install;
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
    Install {game_dir: String},
    Test,
}

fn main() {
    try_main().expect("Unhandled error")
}

fn try_main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Scratch => scratch()?,
        Commands::Dist => dist()?,
        Commands::Install {game_dir} => install(game_dir)?,
        Commands::Test => test()?,
    }

    Ok(())
}
