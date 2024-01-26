use std::{fs, path::PathBuf, str::FromStr};

use anyhow::{Context, Ok, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    minio_address: String,
    listen_port: String,
}

pub fn read_config(config_path: Option<&PathBuf>) -> Result<Config> {
    let default_config_path = PathBuf::from_str("./config.toml").unwrap();
    let path = match config_path {
        Some(path_v) => path_v,
        None => &default_config_path,
    };
    let content = fs::read_to_string(path).context("failed to read config file")?;
    let config = toml::from_str(&content).context("failed to parse config file")?;
    Ok(config)
}
