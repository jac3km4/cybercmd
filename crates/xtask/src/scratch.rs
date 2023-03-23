use anyhow::Result;

use crate::config::Config;

// Temporary code for development testing, a scratch space
pub fn scratch(_: &Config) -> Result<()> {
    println!("Scratch!");

    Ok(())
}
