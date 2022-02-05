use std::{fs::File, io::Read};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::system::Color;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct MeasurrredConfig {
    pub general: GeneralSection,
    pub viewbox_tuning: ViewboxTuningSection,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GeneralSection {
    pub foreground_color: Color,
    pub background_color: Color,
    pub font_family: String,
    pub font_weight: Option<String>,
    pub refresh_interval: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ViewboxTuningSection {
    pub respect_tray_area_when_right_align: bool,
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
