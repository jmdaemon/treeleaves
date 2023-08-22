use std::path::PathBuf;

use crate::{
    consts::{QUALIFIER, ORGANIZATION, APPLICATION},
    cfgfile::ConfigFile
};

use directories::ProjectDirs;
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

pub trait ConfigWith {
    fn path(path: &str) -> ConfigFile {
        let path = PathBuf::from(path);
        let base = path.file_name().unwrap().to_string_lossy();
        let parent = path.parent().unwrap();
        ConfigFile::new(parent, base)
    }

    fn env(name: Option<&str>) -> ConfigFile {
        match name {
            Some(file) => {
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

impl TargetConfigFile {
    pub fn new(path: &str) -> Self {
        let config = <ConfigFile as ConfigWith>::path(path);
        toml::from_str(&config.read()).expect("Could not parse config")
    }

}

impl Default for SharedConfigFile {
    fn default() -> Self {
        let config = <ConfigFile as ConfigWith>::env(option_env!("TREELEAVES_CONFIG_FILE"));
        toml::from_str(&config.read()).expect("Could not parse config")
    }
}

impl SharedConfigFile {
    pub fn new() -> Self {
        SharedConfigFile::default()
    }
}
