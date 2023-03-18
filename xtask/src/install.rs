use std::{env::set_current_dir, path::Path};

use anyhow::Result;
use xshell::{cmd, Shell};

use crate::common::config::PATHS;

pub fn install<P: AsRef<Path>>(game_dir: P) -> Result<()> {
    let sh = Shell::new()?;

    sh.change_dir(&PATHS.root);
    set_current_dir(PATHS.root.as_path())?;

    cmd!(sh, "cargo build --release").run()?;

    println!("Copying cybercmd.asi");
    sh.copy_file(
        PATHS.release.join("cybercmd.dll"),
        game_dir.as_ref().join("bin/x64/plugins/cybercmd.asi"),
    )?;

    println!();
    println!("Done!");

    Ok(())
}
