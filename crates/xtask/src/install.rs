use std::{fs, fs::create_dir_all, path::Path};

use anyhow::Result;
use xshell::Shell;

use crate::{
    config::Config,
    stage::{stage, RELEASE_ARGS},
};

pub fn install<P: AsRef<Path>>(config: &Config<'_>, game_dir: P) -> Result<()> {
    let sh = Shell::new()?;

    println!();
    println!("Start: Building distribution files");
    stage(config, &sh, &RELEASE_ARGS)?;

    recursive_copy(&config.paths.staging, &game_dir, &sh)?;

    println!("Copying cybercmd.asi");
    sh.copy_file(
        config.paths.release.join("cybercmd.dll"),
        game_dir.as_ref().join("bin/x64/plugins/cybercmd.asi"),
    )?;

    println!();
    println!("Done!");

    Ok(())
}

fn recursive_copy<P1: AsRef<Path>, P2: AsRef<Path>>(
    source: &P1,
    dest: &P2,
    sh: &Shell,
) -> Result<()> {
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let new_dest = dest.as_ref().join(entry.file_name());
            create_dir_all(&new_dest)?;
            recursive_copy(&entry.path(), &new_dest, sh)?;
        } else {
            sh.copy_file(entry.path(), dest)?;
        }
    }

    Ok(())
}
