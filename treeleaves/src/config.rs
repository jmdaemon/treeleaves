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
pub struct Config {
    pub url: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SharedConfig {
    #[serde(flatten)]
    cfg: Config
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TargetConfig {
    #[serde(flatten)]
    cfg: Config
}
