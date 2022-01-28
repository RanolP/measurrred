use std::{fs::File, io::Read};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::system::Color;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct MeasurrredConfig {
    pub foreground_color: Color,
    pub background_color: Color,
    pub font_family: String,
    pub font_weight: Option<String>,
    pub refresh_interval: u64,
}

#[derive(Error, Debug)]
pub enum ConfigLoadError {
    #[error("I/O Failed")]
    Io(#[from] std::io::Error),
    #[error("Cannot deserialize toml")]
    TomlDeserialize(#[from] toml::de::Error),
}

impl MeasurrredConfig {
    pub fn load() -> Result<MeasurrredConfig, ConfigLoadError> {
        Ok(toml::from_slice(
            &File::open("measurrred.config.toml")?
                .bytes()
                .collect::<Result<Vec<_>, _>>()?,
        )?)
    }
}
