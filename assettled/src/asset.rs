use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use thiserror::Error;
use time::OffsetDateTime;

pub struct Asset {
    name: String,
    id: String,
}

impl Asset {
    pub fn new(name: String, id: String) -> Self {
        Asset { name, id }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    fn id_normalized(&self) -> String {
        self.id.replace(|c: char| !c.is_alphanumeric(), "")
    }

    pub fn file_path(&self) -> PathBuf {
        Path::new("assets/")
            .join(self.id_normalized())
            .join("file")
            .to_path_buf()
    }

    fn meta_path(&self) -> PathBuf {
        Path::new("assets/")
            .join(self.id_normalized())
            .join("meta.toml")
            .to_path_buf()
    }

    pub fn meta(&self) -> Result<AssetMeta, AssetMetaError> {
        if !self.meta_path().exists() {
            return Ok(Default::default());
        }
        Ok(toml::from_slice(
            &File::open(self.meta_path())
                .and_then(|file| file.bytes().collect::<Result<Vec<_>, _>>())
                .map_err(|e| AssetMetaError::Io(self.id.clone(), e))?,
        )
        .map_err(|e| AssetMetaError::Deserialize(self.id.clone(), e))?)
    }

    pub fn update(&self, data: &[u8]) -> Result<(), AssetMetaError> {
        File::create(self.file_path())
            .and_then(|mut file| file.write_all(data))
            .map_err(|e| AssetMetaError::Io(self.id.clone(), e))?;
        let meta = self.meta()?;
        let meta = toml::to_string(&meta).map_err(|e| AssetMetaError::Serialize(self.id.clone(), e))?;
        File::create(self.meta_path())
            .and_then(|mut file| file.write_all(meta.as_bytes()))
            .map_err(|e| AssetMetaError::Io(self.id.clone(), e))?;
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum AssetMetaError {
    #[error("I/O error while loading metadata from asset {0}: {1}")]
    Io(String, #[source] std::io::Error),
    #[error("Error deserializing metadata from asset {0}: {1}")]
    Deserialize(String, #[source] toml::de::Error),
    #[error("Error serializing metadata from asset {0}: {1}")]
    Serialize(String, #[source] toml::ser::Error),
}

#[derive(Serialize, Deserialize, Default)]
pub struct AssetMeta {
    last_refresh: Option<OffsetDateTime>,
}
