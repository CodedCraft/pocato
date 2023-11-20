// config.rs

use crate::error::ConfigError;
use serde::{Deserialize, Serialize};
use std::{env, fs, path::PathBuf};
use toml;
use once_cell::sync::Lazy;
use crate::config;

pub static CONFIG: Lazy<Config> = Lazy::new(|| config::Config::initialize());

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub nerd_font: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config { nerd_font: false }
    }
}

impl Config {
    pub fn initialize() -> Self {
        match Config::config_handler() {
            Ok(config) => config,
            Err(error) => {
                eprintln!("{error}");
                Config::default()
            }
        }
    }

    fn config_handler() -> Result<Self, ConfigError> {
        let config_dir = Config::get_config_dir()?;
        let mut config_file = config_dir.clone();
        config_file.push("config.toml");

        match (fs::metadata(&config_dir), fs::metadata(&config_file)) {
            // config exists:
            (Ok(_), Ok(_)) => return Config::parse_config(&config_file),
            // only config.toml missing:
            (Ok(_), Err(_)) => return Config::create_config(None, &config_file),
            // pocato dir & config.toml missing:
            (Err(_), Err(_)) => return Config::create_config(Some(&config_dir), &config_file),
            // config.toml can't exist without the containing folder
            (Err(_), Ok(_)) => unreachable!(),
        }
    }

    fn get_config_dir() -> Result<PathBuf, ConfigError> {
        // Check if custom config directory is set with $POCATO_DIR
        if let Ok(pocato_dir) = env::var("POCATO_DIR") {
            let config_dir = PathBuf::from(pocato_dir);
            return Ok(config_dir);
        } else {
            // Linux and MacOS config directory
            #[cfg(not(target_os = "windows"))]
            let config_home = format!("{}/.config", env::var("HOME")?);

            // Windows config directory
            #[cfg(target_os = "windows")]
            let config_home = env::var("APPDATA")?;

            let config_dir = PathBuf::from(format!("{}/pocato", config_home));
            return Ok(config_dir);
        }
    }

    fn create_config(dir: Option<&PathBuf>, file: &PathBuf) -> Result<Self, ConfigError> {
        if let Some(config_dir) = dir {
            fs::create_dir_all(config_dir)?;
        }
        let config = Config::default();
        // let toml_config = toml::to_string(&config)?;
        let toml_config = toml::to_string_pretty(&config)?;
        fs::File::create(file)?;
        fs::write(file, toml_config)?;
        Ok(config)
    }

    fn parse_config(file: &PathBuf) -> Result<Self, ConfigError> {
        let toml_config = fs::read_to_string(file)?;
        let config: Config = toml::from_str(&toml_config)?;
        Ok(config)
    }
}
