use anyhow::Result;
use common::path::PathBuf;

use crate::config::Config;

// Temporary code for development testing, a scratch space
pub fn scratch(config: &Config<'_>) -> Result<()> {
    println!("Scratch!");
    println!();
    println!("Our config: {:#?}", config);
    println!();
    println!("Cargo environment variables:");
    for (key, value) in std::env::vars() {
        if key.starts_with("CARGO") {
            println!("{key}={value}");
        }
    }
    println!("$CARGO compile: {}", env!("CARGO"));
    println!("$CARGO runtime: {}", std::env::var("CARGO")?);

    println!();

    let test_path = PathBuf::new(r"C:\Windows\System\user32.dll")?;
    let components = test_path.components();
    println!("For Path: {:?}", test_path);
    println!("Components: {:#?}", components);
    println!("Iterated:");
    for component in components {
        println!("{:?}", component);
    }

    Ok(())
}
