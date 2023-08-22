use treeleaves::{cfgfile::ConfigFile, config::{SharedConfig, TargetConfig}};

fn main() {
    let config = SharedConfig::new();
    println!("{}", config.shared.url);

    // Target Config
    let file = "config.toml";
    let config_dir = "test/target_data/.treeleaves/";
    let target_config = ConfigFile::new(config_dir, file);
    target_config.make_dirs();

    let config: TargetConfig = toml::from_str(&target_config.read())
        .expect("Could not parse config");

    println!("{}", config.target);
}
