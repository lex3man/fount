use crate::models::Config;

use super::config_file;
use std::io;

#[derive(Debug)]
pub enum ConfigError {
    IoError(io::Error),
    InvalidConfig(toml::de::Error),
}

impl From<io::Error> for ConfigError {
    fn from(value: io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(value: toml::de::Error) -> Self {
        Self::InvalidConfig(value)
    }
}

pub fn init() -> Config {
    let mut config: Config = Config::by_default();

    if config_file().exists() {
        let content = std::fs::read_to_string(config_file()).unwrap();
        config = toml::from_str(&content).unwrap();
    } else {
        save_config(&config).unwrap();
    }

    config
}

pub fn save_config(config: &Config) -> Result<(), ConfigError> {
    let toml_config = toml::to_string(config).unwrap();
    if config_file().exists() {
        std::fs::remove_file(config_file()).unwrap();
    }
    std::fs::write(config_file().to_str().unwrap(), toml_config)?;
    Ok(())
}
