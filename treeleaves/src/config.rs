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

#[derive(Serialize, Deserialize, Debug)]
pub struct SharedConfig {
    #[serde(flatten)]
    pub cfg: Config
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TargetConfig {
    //#[serde(rename = "url")]
    // #[serde(rename(deserialize = "url"))]
    #[serde(flatten)]
    pub cfg: Config
    //pub url: String
}

//impl std::fmt::Display for TargetConfig {
    //fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        //write!(f, "{}", self.url)
    //}
//}

/*
lazy_static!(
    pub static ref TREELEAVES_CONFIG: ConfigFile = create_config("config.toml");
);
*/
