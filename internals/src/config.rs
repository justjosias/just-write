//! Configuration handlers for Just Write
//!
//! The main configuration file is stored in the appropriate platform's
//! `config_dir`, as provided by `directories-rs`.
//!
//! Example `config.toml`:
//! ```
//! root: ~/jw/
//! ```

use std::{
    fs,
    path::{Path, PathBuf},
};

use serde_derive::{Deserialize, Serialize};

use crate::Error;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Config {
    /// The directory where notebooks are stored
    pub root: PathBuf,
}

impl Config {
    pub fn load<P: AsRef<Path> + std::fmt::Debug>(path: P) -> Result<Self, Error> {
        let config_str = fs::read_to_string(path)?;
        let mut config: Config = toml::from_str(config_str.as_str())?;
        config.root = PathBuf::from(expand_tilde(config.root));
        Ok(config)
    }
}

/// Returns the configuration, initializing it if not existent
pub fn get_config() -> Result<Config, Error> {
    let default_config = "";

    let config_path = get_config_path(Some("config.toml"))?;
    if !config_path.exists() {
        fs::write(&config_path, default_config.as_bytes())?;
    }

    let config = Config::load(config_path)?;
    Ok(config)
}

/// Returns the configuration directory based on directories-rs with an optional subpath.
/// Creates the configuration directory if it doesn't exist. Does not create subpath.
pub fn get_config_path(subpath: Option<&str>) -> Result<PathBuf, Error> {
    let dirs = match directories::ProjectDirs::from("dev", "Just Josias", "Just Write") {
        Some(dirs) => dirs,
        None => {
            return Err(Error::Dirs);
        }
    };

    let mut path = dirs.config_dir().to_owned();
    if !path.exists() {
        fs::create_dir_all(&path)?;
    }
    if let Some(p) = subpath {
        path.push(p);
    }
    Ok(path)
}

/// A helper function for expanding a tilde.
/// Assumes the path is a valid UTF-8 string and will panic otherwise.
fn expand_tilde(path: PathBuf) -> String {
    shellexpand::tilde(&path.into_os_string().into_string().unwrap()).into_owned()
}
