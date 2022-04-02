use std::collections::HashMap;

use crate::{Asset, AssetError};

pub struct AssetManager {
    map: HashMap<String, Asset>,
}

impl AssetManager {
    pub fn new() -> Self {
        AssetManager {
            map: HashMap::new(),
        }
    }

    pub fn find(&self, key: &String) -> Option<&Asset> {
        self.map.get(key)
    }

    pub fn display_name(&self, key: &String) -> Option<&str> {
        self.map.get(key).map(|asset| asset.display_name())
    }

    pub fn data(&self, key: &String) -> Result<Option<Vec<u8>>, AssetError> {
        self.map
            .get(key)
            .map(|asset| asset.data())
            .transpose()
            .map(|opt| opt.flatten())
    }

    pub fn update(&mut self, key: String, data: &[u8]) -> Result<(), AssetError> {
        let asset = self
            .map
            .entry(key.clone())
            .or_insert(Asset::new(key.clone(), None)?);
        asset.update(data)?;
        Ok(())
    }

    pub fn rename(&mut self, key: String, name: String) -> Result<(), AssetError> {
        let asset = self
            .map
            .entry(key.clone())
            .or_insert(Asset::new(key.clone(), Some(name.clone()))?);
        asset.rename(name)?;

        Ok(())
    }
}
