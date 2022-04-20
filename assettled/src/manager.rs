use std::collections::{hash_map::Entry, HashMap};

use crate::{Asset, AssetError};

pub struct AssetManager {
    loaded: bool,
    map: HashMap<String, Asset>,
}

impl AssetManager {
    pub fn new() -> Self {
        AssetManager {
            loaded: false,
            map: HashMap::new(),
        }
    }

    pub fn has_loaded(&self) -> bool {
        self.loaded
    }

    pub async fn assume_loaded(&mut self) -> Result<(), AssetError> {
        if self.loaded {
            return Ok(());
        }
        for asset in Asset::load().await? {
            self.map.insert(asset.id(), asset);
        }

        self.loaded = true;

        Ok(())
    }

    pub fn find(&self, key: &String) -> Option<&Asset> {
        self.map.get(key)
    }

    pub fn display_name(&self, key: &String) -> Option<&str> {
        self.map.get(key).map(|asset| asset.display_name())
    }

    pub async fn data(&self, key: &String) -> Result<Option<Vec<u8>>, AssetError> {
        if let Some(asset) = self.map.get(key) {
            asset.data().await
        } else {
            Ok(None)
        }
    }

    pub async fn update(&mut self, key: String, data: &[u8]) -> Result<(), AssetError> {
        let entry = self.map.entry(key.clone());
        let asset = match entry {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(Asset::new(key.clone(), None).await?),
        };
        asset.update(data).await?;
        Ok(())
    }

    pub async fn rename(&mut self, key: String, name: String) -> Result<(), AssetError> {
        let entry = self.map.entry(key.clone());
        let asset = match entry {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(Asset::new(key.clone(), None).await?),
        };
        asset.rename(name).await?;

        Ok(())
    }
}
