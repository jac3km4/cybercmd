use std::path::Path;

use cybercmd::{config::get_configs, paths::PATHS, util::setup_logging};

pub fn main() {
    setup_logging().expect("Logger setup failed!");

    log::info!("Running \"main.exe\" test app.");

    println!(
        "{:?}",
        Path::join(Path::new(&PATHS.game), "something else/entirely")
    );

    let configs = match get_configs() {
        Ok(configs) => configs,
        Err(error) => panic!("{:?}", error),
    };

    for config in configs {
        log::debug!("Loaded config: {:?}", config);
    }
}
