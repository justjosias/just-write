//! Configuration handlers for Just Write
//!
//! The main configuration file is stored in the appropriate platform's
//! `config_dir`, as provided by `directories-rs`.
//!
//! Example `config.toml`:
//! ```toml
//! root = "~/jw/"
//! ```

use std::{
    fs,
    path::{Path, PathBuf},
};

use serde_derive::{Deserialize, Serialize};

use crate::Error;

#[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct Config {
    /// The directory where notebooks are stored
    pub root: PathBuf,
}

#[derive(Debug, Default, Deserialize, Serialize)]
struct TempConfig {
    root: String,
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        Self::from_str(&fs::read_to_string(path)?)
    }

    fn from_str(s: &str) -> Result<Self, Error> {
        let config: TempConfig = toml::from_str(s)?;
        let root = shellexpand::tilde(&config.root).into_owned();
        Ok(Config {
            root: PathBuf::from(root),
        })
    }
}

#[test]
fn load_from_str() {
    let s = r#"root = "~/our/nice/directory""#;
    let home = std::env::var("HOME").unwrap();
    assert_eq!(
        Config::from_str(s).unwrap(),
        Config {
            root: PathBuf::from(format!("{home}/our/nice/directory")),
        }
    );
}

/// Returns the configuration, initializing it if not existent
pub fn get() -> Result<Config, Error> {
    let default_config = "";

    let config_path = get_path(Some("config.toml"))?;
    if !config_path.exists() {
        fs::write(&config_path, default_config.as_bytes())?;
    }

    let config = Config::load(config_path)?;
    Ok(config)
}

/// Returns the configuration directory based on directories-rs with an optional subpath.
/// Creates the configuration directory if it doesn't exist. Does not create subpath.
pub fn get_path(subpath: Option<&str>) -> Result<PathBuf, Error> {
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
