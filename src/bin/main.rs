use cybercmd::ModConfig;

pub fn main() {
    let contents = std::fs::read_to_string("./reference/cmd.toml").unwrap();
    let config: ModConfig = toml::from_str(&contents).unwrap();
    dbg!(config);
}
