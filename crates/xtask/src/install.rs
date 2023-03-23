use std::{env::set_current_dir, fs, path::Path};

use anyhow::Result;
use xshell::{cmd, Shell};

use crate::config::Config;

pub fn install<P: AsRef<Path>>(config: &Config, game_dir: P) -> Result<()> {
    let sh = Shell::new()?;

    sh.change_dir(&config.paths.root);
    set_current_dir(config.paths.root.as_path())?;

    cmd!(sh, "cargo build --release --package cybercmd").run()?;

    println!("Adding config files (redscript)");
    for path in fs::read_dir(&config.paths.config)? {
        sh.copy_file(path?.path(), game_dir.as_ref().join("r6/config/cybercmd"))?;
    }

    println!("Copying cybercmd.asi");
    sh.copy_file(
        config.paths.release.join("cybercmd.dll"),
        game_dir.as_ref().join("bin/x64/plugins/cybercmd.asi"),
    )?;

    println!();
    println!("Done!");

    Ok(())
}
