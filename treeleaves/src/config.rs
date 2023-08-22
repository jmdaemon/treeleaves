use crate::consts::{QUALIFIER, ORGANIZATION, APPLICATION};
use cfg::{ConfigFile, format_config_path, FallbackExt};
use directories::ProjectDirs;
use serde::{Serialize, Deserialize};

// Manage Treeleaves configs

pub fn create_project_dirs() -> ProjectDirs {
    ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION)
        .expect("Could not initialize project directories")
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

impl Default for SharedConfigFile {
    fn default() -> Self {
        let config = ConfigFile::from_option_env("TREELEAVES_CONFIG_FILE")
            .fallback_path(format_config_path(create_project_dirs(), "config.toml"));
        toml::from_str(&config.read()).expect("Could not parse config")
    }
}

impl SharedConfigFile {
    pub fn new() -> Self {
        SharedConfigFile::default()
    }
}

impl TargetConfigFile {
    pub fn new(path: &str) -> Self {
        let config = ConfigFile::new(path);
        toml::from_str(&config.read()).expect("Could not parse config")
    }
}
