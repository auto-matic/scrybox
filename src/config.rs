use anyhow::Result;
use serde::Deserialize;
use std::{fs, path::PathBuf};

#[derive(Deserialize)]
pub struct Config {
    pub files: Files,
    pub db: DB,
}

#[derive(Deserialize)]
pub struct Files {
    pub collection: PathBuf,
    pub scryfall_data: PathBuf,
    pub db: PathBuf,
}

#[derive(Deserialize)]
pub struct DB {
    pub overwrite_old: bool,
}

pub fn load_config() -> Result<Config> {
    Ok(if fs::exists("./config.toml")? {
        let config_str = fs::read_to_string("./config.toml")?;
        toml::from_str(&config_str)?
    } else {
        let default_config = include_str!("./default_config.toml");
        fs::write("./config.toml", default_config)?;
        toml::from_str(default_config)?
    })
}
