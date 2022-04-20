use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    pin::Pin,
};

use async_std::sync::RwLock;
use futures::Future;
use thiserror::Error;
use url::Url;

use crate::{AssetError, AssetManager};

pub struct AssetPreHook<E> {
    pub manager: RwLock<AssetManager>,
    pub fetch:
        Box<dyn Fn(Url) -> Pin<Box<dyn Future<Output = Result<Vec<u8>, E>> + Send>> + Sync + Send>,
}

#[derive(Debug, Error)]
pub enum AssetPreHookError<E> {
    AssetError(#[source] AssetError),
    Custom(#[from] E),
}

impl<E> AssetPreHook<E> {
    pub async fn run(&self, url: Url) -> Result<Vec<u8>, AssetPreHookError<E>> {
        let mut hasher = DefaultHasher::new();
        url.hash(&mut hasher);
        let key = format!("{}-{:x}", url.domain().unwrap_or_default(), hasher.finish());
        {
            let manager_read = self.manager.read().await;
            let manager_read = if !manager_read.has_loaded() {
                drop(manager_read);
                self.manager.write().await.assume_loaded().await.map_err(AssetPreHookError::AssetError)?;
                self.manager.read().await
            } else {
                manager_read
            };
            if let Some(cache) = manager_read.find(&key) {
                let data = cache.data().await.ok().flatten();
                if let Some(data) = data {
                    return Ok(data);
                }
            }
        }

        let body = (self.fetch)(url).await?;

        let mut manager_write = self.manager.write().await;
        manager_write
            .update(key, &body)
            .await
            .map_err(AssetPreHookError::AssetError)?;

        Ok(body)
    }
}
