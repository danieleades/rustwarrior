use std::{
    fs::{self, File},
    io::{self, BufReader, Read},
    path::{Path, PathBuf},
};

use rustwarrior::task::Coefficients;
use serde::{Deserialize, Serialize};

const CONFIG_FILE: &str = "config.toml";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    #[serde(default)]
    pub coefficients: Coefficients,
}

impl Config {
    pub fn load() -> Result<Self, Error> {
        let config_path = default_config_dir()?.join(CONFIG_FILE);
        load_config_from_file(config_path)
    }
}

fn default_config_dir() -> io::Result<PathBuf> {
    let dir = dirs::config_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "couldn't find data directory"))?
        .join("rustwarrior");

    fs::create_dir_all(&dir)?;
    Ok(dir)
}

fn load_config_from_file(path: impl AsRef<Path>) -> Result<Config, Error> {
    let config_file = File::options()
        .create(true)
        .write(true)
        .read(true)
        .open(path)?;
    let mut reader = BufReader::new(config_file);
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    Ok(toml::from_slice(&buf)?)
}

#[derive(Debug, thiserror::Error)]
#[error("Failed to load tasks from file: {0}")]
pub enum Error {
    Toml(#[from] toml::de::Error),
    Io(#[from] io::Error),
}
