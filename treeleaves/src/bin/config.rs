//use treeleaves::config::TREELEAVES_CONFIG;
use treeleaves::{cfgfile::ConfigFile, config::{SharedConfig, TargetConfig, create_config}};
use toml::{Value, from_str};
use serde::Deserialize;
use indexmap::IndexMap;
//use lazy_static::lazy_static;

//lazy_static!(
    //static ref TREELEAVES_CONFIG: ConfigFile = create_config("config.toml");
//);

#[derive(Deserialize)]
pub struct TreeleavesConfig {
    pub shared: SharedConfig,
}

#[derive(Deserialize)]
pub struct TreeleavesTargetConfig {
    pub target: TargetConfig,
}


fn main() {
    // Shared Config
    let treeleaves_config = create_config("config.toml");
    treeleaves_config.make_dirs();

    let config: TreeleavesConfig = toml::from_str(&treeleaves_config.read())
        .expect("Could not parse config");

    println!("{}", config.shared.cfg);

    // Target Config
    let file = "config.toml";
    //let config_dir = format!("test/target_data/.treeleaves/{}", file);
    let config_dir = "test/target_data/.treeleaves/";
    let target_config = ConfigFile::new(config_dir, file);
    target_config.make_dirs();

    let config: TreeleavesTargetConfig = toml::from_str(&target_config.read())
        .expect("Could not parse config");

    //println!("{}", config.cfg);
    println!("{}", config.target.cfg);
}
