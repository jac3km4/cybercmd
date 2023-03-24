use std::fs;

use anyhow::Result;
use xshell::{cmd, Shell};

use crate::{
    config::Config,
    stage::{stage, stage_add_standalone},
};

pub fn test(config: &Config<'_>) -> Result<()> {
    println!();
    println!("Start: Running Tester");
    let sh = &Shell::new()?;

    stage(
        config,
        sh,
        &vec![
            "--package",
            "test",
            "-Z",
            "build-std=std,panic_abort",
            "-Z",
            "build-std-features=panic_immediate_abort",
            "--release",
        ],
    )?;
    stage_add_standalone(config)?;

    println!("Adding config files from examples.");
    for path in fs::read_dir(&config.paths.examples)? {
        sh.copy_file(path?.path(), &config.paths.staging_config)?;
    }

    println!("Copying test.exe");
    let test_exe = &config.paths.staging_bin.join("test.exe");
    sh.copy_file(config.paths.debug.join("test.exe"), test_exe)?;

    println!("Running test.exe");
    sh.change_dir(&config.paths.staging_bin);
    cmd!(sh, "{test_exe}").run()?;

    println!("Done:  Running Tester");

    Ok(())
}
