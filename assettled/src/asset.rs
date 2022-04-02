use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use thiserror::Error;
use time::OffsetDateTime;

pub struct AssetId(String);

impl AssetId {
    fn normalized(&self) -> String {
        self.0.replace(|c: char| !c.is_alphanumeric(), "")
    }

    fn asset_path(&self) -> PathBuf {
        Path::new("assets/").join(self.normalized()).to_path_buf()
    }

    fn data_path(&self) -> PathBuf {
        self.asset_path().join("data")
    }

    fn meta_path(&self) -> PathBuf {
        self.asset_path().join("meta.toml")
    }

    fn load_meta(&self) -> Result<Option<AssetMeta>, AssetError> {
        if !self.meta_path().exists() {
            return Ok(None);
        }
        Ok(toml::from_slice(
            &File::open(self.meta_path())
                .and_then(|file| file.bytes().collect::<Result<Vec<_>, _>>())
                .map_err(|e| AssetError::Io(self.0.clone(), e))?,
        )
        .map_err(|e| AssetError::DeserializeMeta(self.0.clone(), e))?)
    }
}

pub struct Asset {
    id: AssetId,
    meta: AssetMeta,
}

impl Asset {
    pub fn new(id: String, name: Option<String>) -> Result<Self, AssetError> {
        let id = AssetId(id);
        let mut meta = id.load_meta()?.unwrap_or(AssetMeta {
            last_refresh: None,
            name: name.clone(),
        });
        meta.name = name.clone();
        Ok(Asset { id, meta })
    }

    pub fn display_name(&self) -> &str {
        &self.meta.name.as_ref().unwrap_or(&self.id.0)
    }

    pub fn has_name(&self) -> bool {
        self.meta.name.is_some()
    }

    pub fn data_path(&self) -> PathBuf {
        self.id.data_path()
    }

    pub fn meta(&self) -> &AssetMeta {
        &self.meta
    }

    pub fn data(&self) -> Result<Option<Vec<u8>>, AssetError> {
        if !self.data_path().exists() {
            return Ok(None);
        }
        let mut output = Vec::new();
        File::open(self.data_path())
            .and_then(|mut file| file.read_to_end(&mut output))
            .map_err(|e| AssetError::Io(self.id.0.clone(), e))?;
        Ok(Some(output))
    }

    pub fn rename(&mut self, name: String) -> Result<(), AssetError> {
        self.meta.name = Some(name);
        self.save_meta()
    }

    pub fn update(&self, data: &[u8]) -> Result<(), AssetError> {
        File::create(self.data_path())
            .and_then(|mut file| file.write_all(data))
            .map_err(|e| AssetError::Io(self.id.0.clone(), e))?;
        self.save_meta()
    }

    fn save_meta(&self) -> Result<(), AssetError> {
        let meta_str = toml::to_string(&self.meta)
            .map_err(|e| AssetError::SerializeMeta(self.id.0.clone(), e))?;
        File::create(self.id.meta_path())
            .and_then(|mut file| file.write_all(meta_str.as_bytes()))
            .map_err(|e| AssetError::Io(self.id.0.clone(), e))?;
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum AssetError {
    #[error("I/O error from asset {0}: {1}")]
    Io(String, #[source] std::io::Error),
    #[error("Error deserializing metadata from asset {0}: {1}")]
    DeserializeMeta(String, #[source] toml::de::Error),
    #[error("Error serializing metadata from asset {0}: {1}")]
    SerializeMeta(String, #[source] toml::ser::Error),
}

#[derive(Serialize, Deserialize)]
pub struct AssetMeta {
    last_refresh: Option<OffsetDateTime>,
    name: Option<String>,
}
