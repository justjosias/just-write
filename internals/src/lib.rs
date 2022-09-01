pub mod config;
pub mod notebooks;
pub mod search;

mod time;

pub use config::Config;
pub use notebooks::Notebook;

use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("IO Error!")]
    IO(#[from] std::io::Error),
    #[error("Parse Error!")]
    Deserialize(#[from] toml::de::Error),
    #[error("Parse Error!")]
    Serialize(#[from] toml::ser::Error),
    #[error("Dirs Error!")]
    Dirs,
    #[error("Exists Error!")]
    Exists,
}
