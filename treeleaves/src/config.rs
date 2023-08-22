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

//#[derive(Serialize, Deserialize, Debug)]
//pub struct Config {
    //pub url: String
//}

//impl std::fmt::Display for Config {
    //fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        //write!(f, "{}", self.url)
    //}
//}

// Requirements:
// 
// The ability to 

//#[derive(Serialize, Deserialize, Debug)]
//pub struct SharedConfig {
    //pub shared: Config,
//}

//#[derive(Serialize, Deserialize, Debug)]
//pub enum Config {
    //#[serde(rename = "shared")]
    //Shared {
        //url: String,
    //},
    //#[serde(rename = "target")]
    //Target {
        //url: String,
    //},
//}

#[derive(Serialize, Deserialize, Debug)]
pub struct DatabaseConfig {
    pub url: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SharedConfigFile {
    pub shared: DatabaseConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TargetConfigFile {
    pub target: DatabaseConfig,
}

//pub const TREELEAVES_CONFIG_FILE: &str = "config.toml";
//pub const TREELEAVES_CONFIG_FILE: Option<&str> = option_env!("TREELEAVES_CONFIG_FILE");
//pub const DEFAULT_CONFIG_NAME: &str = "config.toml";

// Generate the 
//pub trait ConfigFilePathsExt {
//pub trait Configure {
pub trait ConfigWith {
    //fn with_config_from(path: &str) -> ConfigFile {
    fn path(path: &str) -> ConfigFile {
        let path = PathBuf::from(path);
        let base = path.file_name().unwrap().to_string_lossy();
        let parent = path.parent().unwrap();
        ConfigFile::new(parent, base)
    }

    //fn with_env(name: Option<&str>) -> ConfigFile {
    fn env(name: Option<&str>) -> ConfigFile {
        match name {
            Some(file) => {
                //<ConfigFile as ConfigFilePathsExt>::with_config_from(file)
                <ConfigFile as ConfigWith>::path(file)
            }
            None => {
                let project_dirs = create_project_dirs();
                let base = "config.toml";
                let parent = project_dirs.config_dir();
                ConfigFile::new(parent, base)
            }
        }
    }
}

impl ConfigWith for ConfigFile { }
//impl ConfigFilePathsExt for ConfigFile { }

impl TargetConfigFile {
    pub fn new(path: &str) -> Self {
        let config = <ConfigFile as ConfigWith>::path(path);
        let config: TargetConfigFile = toml::from_str(&config.read())
            .expect("Could not parse config");
        config
    }

}

impl Default for SharedConfigFile {
    fn default() -> Self {
        let config = <ConfigFile as ConfigWith>::env(option_env!("TREELEAVES_CONFIG_FILE"));
        let config: SharedConfigFile = toml::from_str(&config.read())
            .expect("Could not parse config");
        config
    }
}

impl SharedConfigFile {
    pub fn new() -> Self {
        SharedConfigFile::default()
    }
}

//impl Config {
/*
impl SharedConfigFile {
    //pub fn new(dir: PathBuf, file: String) -> Self {
    pub fn new() -> Self {
        // Get the config file paths
        //let config = match TREELEAVES_CONFIG_FILE {
        let config = match option_env!("TREELEAVES_CONFIG_FILE") {
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
        let config: SharedConfigFile = toml::from_str(&config.read())
            .expect("Could not parse config");
        config

        //let config = TREELEAVES_CONFIG_FILE.map_or(format!("{}/{}", project_dirs.config_dir(), DEFAULT_CONFIG_NAME));
        //let config = ConfigFile::new(project_dirs.config_dir(), TREELEAVES_CONFIG_FILE);

    //TREELEAVES_SHARED_CONFIG.make_dirs();
    //let config: SharedConfig = toml::from_str(&TREELEAVES_SHARED_CONFIG.read())
        //.expect("Could not parse config");
    }
}

impl TargetConfigFile {
    pub fn new(path: &str) -> Self {
        let path = PathBuf::from(path);
        let base = path.file_name().unwrap().to_string_lossy();
        let parent = path.parent().unwrap();
        let config = ConfigFile::new(parent, base);

        // Deserialize the config
        let config: TargetConfigFile = toml::from_str(&config.read())
            .expect("Could not parse config");
        config
    }
}
*/

//#[derive(Serialize, Deserialize, Debug)]
//pub struct TargetConfig {
    //pub target: Config,
//}

//lazy_static!(
    //pub static ref TREELEAVES_SHARED_CONFIG: ConfigFile = create_config("config.toml");
//);
