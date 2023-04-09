use anyhow::Result;
use common::file::zip_files;
use uniquote::Quote;
use xshell::Shell;

use crate::{
    config::Config,
    stage::{stage, stage_add_standalone, stage_fomod, RELEASE_ARGS},
};

pub fn dist(config: &Config<'_>) -> Result<()> {
    let sh = Shell::new()?;

    println!();
    println!("Start: Building distribution files");
    stage(config, &sh, &RELEASE_ARGS)?;
    stage_fomod(config, &sh)?;

    println!("Cleanup dist");
    config.paths.clean_dist()?;

    let main_zip = &config.paths.dist.join("cybercmd.zip");
    println!("Creating: {}", main_zip.quote());
    zip_files(&config.paths.staging, main_zip)?;

    stage_add_standalone(config)?;

    let standalone_zip = config.paths.dist.join("cybercmd-standalone.zip");
    println!("Creating: {}", standalone_zip.quote());
    zip_files(&config.paths.staging, standalone_zip)?;

    println!("Done:  Building distribution files");

    Ok(())
}
