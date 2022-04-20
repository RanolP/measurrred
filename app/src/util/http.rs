use assettled::{AssetManager, AssetPreHook, AssetPreHookError};
use async_std::sync::RwLock;
use once_cell::sync::Lazy;
use surf::{Client,Url};

static CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::new()
});
static HOOK: Lazy<AssetPreHook<surf::Error>> = Lazy::new(|| {
    AssetPreHook {
        manager: RwLock::new(AssetManager::new()),
        fetch: Box::new(|url| {
            Box::pin(async { Ok(CLIENT.get(url).send().await?.body_bytes().await?) })
        }),
    }
});

pub async fn get(url: impl AsRef<str>) -> Result<Vec<u8>, AssetPreHookError<surf::Error>> {
    HOOK.run(Url::parse(url.as_ref()).map_err(surf::Error::from)?).await
}
