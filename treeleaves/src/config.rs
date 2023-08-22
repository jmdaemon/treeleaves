use std::path::PathBuf;

use crate::{
    consts::{QUALIFIER, ORGANIZATION, APPLICATION},
    cfgfile::ConfigFile
};

use directories::ProjectDirs;
use lazy_static::lazy_static;
use serde::{Serialize, Deserialize};

pub fn create_project_dirs() -> ProjectDirs {
    ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION)
        .expect("Could not initialize project directories")
}

pub fn create_config(file: impl Into<String>) -> ConfigFile {
    let project_dirs = create_project_dirs();
    let config_dir = project_dirs.config_dir();
    ConfigFile::new(config_dir, file)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub url: String
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.url)
    }
}

// Requirements:
// 
// The ability to 

#[derive(Serialize, Deserialize, Debug)]
pub struct SharedConfig {
    pub shared: Config,
}

//pub const TREELEAVES_CONFIG_FILE: &str = "config.toml";
pub const TREELEAVES_CONFIG_FILE: Option<&str> = option_env!("TREELEAVES_CONFIG_FILE");
//pub const DEFAULT_CONFIG_NAME: &str = "config.toml";

impl SharedConfig {
    //pub fn new(dir: PathBuf, file: String) -> Self {
    pub fn new() -> Self {
        // Get the config file paths
        let config = match TREELEAVES_CONFIG_FILE {
            Some(file) =>  {
                let path = PathBuf::from(file);
                let base = path.file_name().unwrap().to_string_lossy();
                let parent = path.parent().unwrap();
                ConfigFile::new(parent, base)
            }
            None => {
                let project_dirs = create_project_dirs();
                let base = "config.toml";
                let parent = project_dirs.config_dir();
                ConfigFile::new(parent, base)
            }
        };
        // Deserialize the config
        let config: SharedConfig = toml::from_str(&config.read())
            .expect("Could not parse config");
        config

        //let config = TREELEAVES_CONFIG_FILE.map_or(format!("{}/{}", project_dirs.config_dir(), DEFAULT_CONFIG_NAME));
        //let config = ConfigFile::new(project_dirs.config_dir(), TREELEAVES_CONFIG_FILE);

    //TREELEAVES_SHARED_CONFIG.make_dirs();
    //let config: SharedConfig = toml::from_str(&TREELEAVES_SHARED_CONFIG.read())
        //.expect("Could not parse config");
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TargetConfig {
    pub target: Config,
}

//lazy_static!(
    //pub static ref TREELEAVES_SHARED_CONFIG: ConfigFile = create_config("config.toml");
//);
