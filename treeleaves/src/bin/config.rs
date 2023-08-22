use treeleaves::{cfgfile::ConfigFile, config::{SharedConfig, TargetConfig, TREELEAVES_SHARED_CONFIG}};

fn main() {
    // Shared Config
    TREELEAVES_SHARED_CONFIG.make_dirs();
    let config: SharedConfig = toml::from_str(&TREELEAVES_SHARED_CONFIG.read())
        .expect("Could not parse config");

    println!("{}", config.shared);

    // Target Config
    let file = "config.toml";
    let config_dir = "test/target_data/.treeleaves/";
    let target_config = ConfigFile::new(config_dir, file);
    target_config.make_dirs();

    let config: TargetConfig = toml::from_str(&target_config.read())
        .expect("Could not parse config");

    println!("{}", config.target);
}
