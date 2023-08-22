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

pub trait FallbackExt<T> {
    fn fallback(self, fallback: impl Into<String>) -> T;
    fn fallback_path(self, fallback: impl Into<PathBuf>) -> T;
}

impl FallbackExt<ConfigFile> for Option<ConfigFile> {
    fn fallback(self: Option<ConfigFile>, fallback: impl Into<String>) -> ConfigFile {
        self.unwrap_or(ConfigFile::new(fallback.into()))
    }

    fn fallback_path(self: Option<ConfigFile>, fallback: impl Into<PathBuf>) -> ConfigFile {
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

    pub fn from_option_env(env_name: &str) -> Option<Self> {
        env::var(env_name).ok().map(ConfigFile::new)
    }

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

pub fn format_path(config_dir: &Path, config: impl Into<String>) -> PathBuf {
    config_dir.join(config.into())
}

pub fn format_config_path(project_dirs: ProjectDirs, file: impl Into<String>) -> PathBuf {
    format_path(project_dirs.config_dir(), file.into())
}
