use std::{
    env,
    fs::{create_dir_all, read_to_string, write},
    path::{PathBuf, Path},
    str::FromStr,
    io,
};

use directories::ProjectDirs;

/// Manage application config files
#[derive(Debug, Default, Clone)]
pub struct ConfigFile {
    pub path: PathBuf,
    pub config_dir: PathBuf,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParsePathError;

impl FromStr for ConfigFile {
    type Err = ParsePathError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let path = PathBuf::from(s);
        Ok(ConfigFile::new(path))
    }
}

//pub trait FallbackExt<T, V> {
pub trait FallbackExt<T> {
    //type Self = Option<T>;
    //fn fallback(opt_self: Option<T>, fallback: &str) -> T;
    //fn fallback(self: Option<T>, fallback: &str) -> T;
    //fn fallback(self: Option<T>, fallback: &str) -> T;
    //fn fallback(self: Option<Box<Self>>, fallback: &str) -> Self;
    //fn fallback(self: Option<T>, fallback: &str) -> Self;
    //fn fallback(self: Option<T>, fallback: &str) -> Self;
    //fn fallback(self, fallback: &str) -> Self;
    //fn fallback(self, fallback: &str) -> T;
    fn fallback(self, fallback: impl Into<String>) -> T;
    fn fallback_path(self, fallback: impl Into<PathBuf>) -> T;
}

//impl FallbackExt<ConfigFile> for Option<ConfigFile> {
//impl FallbackExt<Option<ConfigFile>, ConfigFile> for Option<ConfigFile> {
impl FallbackExt<ConfigFile> for Option<ConfigFile> {
    //fn fallback(opt_self: Option<ConfigFile>, fallback: &str) -> ConfigFile {
    //fn fallback(self: Option<ConfigFile>, fallback: &str) -> ConfigFile {
    //fn fallback(self: Option<ConfigFile>, fallback: &str) -> Self {
    //fn fallback(self: Option<ConfigFile>, fallback: &str) -> ConfigFile {

    fn fallback(self: Option<ConfigFile>, fallback: impl Into<String>) -> ConfigFile {
        //self.unwrap_or(ConfigFile::new(fallback))
        self.unwrap_or(ConfigFile::new(fallback.into()))
    }
    fn fallback_path(self: Option<ConfigFile>, fallback: impl Into<PathBuf>) -> ConfigFile {
        //self.unwrap_or(ConfigFile::new(fallback))
        self.unwrap_or(ConfigFile::new(fallback.into()))
    }
}

impl ConfigFile {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        let config_dir = path.parent().unwrap().to_owned();
        Self { path, config_dir }
    }

    pub fn from_project_dirs(project_dirs: ProjectDirs, file: impl Into<String>) -> Self{
        let path = format_path(project_dirs.config_dir(), file.into());
        Self { path, config_dir: project_dirs.config_dir().to_owned() }
    }

    //pub fn from_option_env(env: Option<&str>, default: &str) -> Option<Self> {
    //pub fn from_option_env(env: Option<&str>) -> Option<Self> {
    //pub fn from_option_env(env_name: impl Into<String>) -> Option<Self> {
    pub fn from_option_env(env_name: &str) -> Option<Self> {
        env::var(env_name).ok().map(ConfigFile::new)
            //.map(Result::Ok)
            //.map(ConfigFile::new)

        //env::var(env_name).map(ConfigFile::new)
        //option_env!(env_name).map(ConfigFile::new)
        //env.map(ConfigFile::new)
        //match env {
            //Some(env_path) => Some(ConfigFile::new(env_path)),
            ////None => ConfigFile::new(default),
            //None => None,
        //}
    }
    //pub fn fallback(self: Option<Self>) {
    //pub fn fallback(opt_self: Option<Self>, path: impl Into<PathBuf>) -> Self {
    //pub fn fallback(self as Option<Self>, path: impl Into<PathBuf>) -> Self {
    //pub fn fallback(self: Option<Self>, path: impl Into<PathBuf>) -> Self {

    //pub fn fallback(self: Box<Option<Self>>, path: impl Into<PathBuf>) -> Self {
        //opt_self.unwrap_or(ConfigFile::new(path.into()))
    //}

    pub fn make_dirs(&self) -> Self {
        create_dir_all(&self.config_dir).expect("Could not create config directory");
        self.to_owned()
    }

    pub fn read(&self) -> String {
        read_to_string(&self.path).expect("Could not read config file.")
    }

    pub fn write(&self, conts: &str) -> io::Result<()> {
        write(&self.path, conts)
    }
}

//pub fn format_config_path(config_dir: &Path, config: impl Into<String>) -> PathBuf {
pub fn format_path(config_dir: &Path, config: impl Into<String>) -> PathBuf {
    config_dir.join(config.into())
}

pub fn format_config_path(project_dirs: ProjectDirs, file: impl Into<String>) -> PathBuf {
    format_path(project_dirs.config_dir(), file.into())
}
