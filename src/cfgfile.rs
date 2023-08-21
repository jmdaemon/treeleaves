use std::{
    fs::{create_dir_all, read_to_string, write},
    path::{PathBuf, Path},
};

/// Manage application config files
#[derive(Debug, Default, Clone)]
pub struct ConfigFile {
    pub config_dir: PathBuf,
    pub path: PathBuf,
}

impl ConfigFile {
    pub fn new(config_dir: impl Into<PathBuf>, file: impl Into<String>) -> Self {
        let config_dir = config_dir.into();
        let path = ConfigFile::format_path(&config_dir, file.into());
        Self { config_dir, path }
    }

    fn format_path(config_dir: &Path, config: impl Into<String>) -> PathBuf {
        config_dir.join(config.into())
    }

    pub fn make_dirs(&self) {
        create_dir_all(&self.config_dir).expect("Could not create config directory")
    }

    pub fn read(&self) -> String {
        read_to_string(&self.path).expect("Could not read config file.")
    }

    pub fn write(&self, conts: &str) -> std::io::Result<()> {
        write(&self.path, conts)
    }
}
