use cybercmd::get_configs;
use cybercmd::logger::Logger;

pub fn main() {
    std::env::set_var("CYBERCMD_DEBUG", "5");
    let mut log = Logger::new();

    log.info("Running \"main.exe\" test app.");

    let configs = match get_configs(&mut log) {
        Ok(configs) => configs,
        Err(error) => panic!("{:?}", error),
    };

    for config in configs {
        log.debug(format!("Loaded config: {:?}", config));
    }
}
