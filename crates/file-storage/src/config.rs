use std::{fs, path::PathBuf, str::FromStr};

use anyhow::{Context, Ok, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Minio {
    pub ip: String,
    pub port: u16,
    pub access_key: String,
    pub secret_key: String,
    pub bucket: String,
}

#[derive(Debug, Deserialize)]
pub struct Service {
    pub listen: u16,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub minio: Minio,
    pub service: Service,
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
