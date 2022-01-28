use futures::executor;
use http_cache::{CACacheManager, CacheMode, HttpCache};
use http_cache_surf::Cache;
use once_cell::sync::Lazy;
use surf::{Client, Result};

static CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::new().with(Cache(HttpCache {
        mode: CacheMode::Default,
        manager: CACacheManager {
            path: "./.cache/http".to_string(),
        },
        options: None,
    }))
});

pub fn get(url: impl AsRef<str>) -> Result<Vec<u8>> {
    executor::block_on(get_async(url))
}

async fn get_async(url: impl AsRef<str>) -> Result<Vec<u8>> {
    let mut response = CLIENT.get(url).send().await?;
    Ok(response.body_bytes().await?)
}
