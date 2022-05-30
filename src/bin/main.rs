use cybercmd::ModConfig;

pub fn main() {
    let contents = std::fs::read("./reference/cmd.toml").unwrap();
    let config: ModConfig = toml::from_slice(&contents).unwrap();
    dbg!(config);
}
