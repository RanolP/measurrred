use std::path::{Path, PathBuf};

use async_std::{
    fs::{create_dir_all, File},
    io::ReadExt,
};
use futures::{future::try_join_all, AsyncWriteExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use time::OffsetDateTime;

pub struct AssetId(String);

impl AssetId {
    fn list() -> Vec<String> {
        Path::new(".assets/")
            .read_dir()
            .into_iter()
            .flatten()
            .flat_map(|res| res.into_iter())
            .flat_map(|entry| {
                entry
                    .path()
                    .components()
                    .last()
                    .map(|s| s.as_os_str().to_string_lossy().into_owned())
                    .into_iter()
            })
            .collect()
    }

    fn io_error(&self, error: std::io::Error) -> AssetError {
        AssetError::Io(self.0.clone(), error)
    }

    fn asset_path(&self) -> PathBuf {
        Path::new(".assets/").join(&self.0).to_path_buf()
    }

    fn data_path(&self) -> PathBuf {
        self.asset_path().join("data")
    }

    fn meta_path(&self) -> PathBuf {
        self.asset_path().join("meta.toml")
    }

    async fn load_meta(&self) -> Result<Option<AssetMeta>, AssetError> {
        if !self.meta_path().exists() {
            return Ok(None);
        }
        let file = File::open(self.meta_path())
            .await
            .map_err(|e| self.io_error(e))?;
        let bytes: Vec<u8> = file
            .bytes()
            .try_collect::<Vec<_>>()
            .await
            .map_err(|e| self.io_error(e))?;
        Ok(Some(toml::from_slice(&bytes).map_err(|e| {
            AssetError::DeserializeMeta(self.0.clone(), e)
        })?))
    }
}

pub struct Asset {
    id: AssetId,
    meta: AssetMeta,
}

impl Asset {
    pub async fn load() -> Result<Vec<Asset>, AssetError> {
        try_join_all(
            AssetId::list()
                .into_iter()
                .map(|id| Asset::new(id, None))
                .collect::<Vec<_>>(),
        ).await
    }

    pub async fn new(id: String, name: Option<String>) -> Result<Self, AssetError> {
        let id = AssetId(id);
        let mut meta = id.load_meta().await?.unwrap_or(AssetMeta {
            last_refresh: None,
            name: name.clone(),
        });
        meta.name = name.clone();
        Ok(Asset { id, meta })
    }

    pub fn id(&self) -> String {
        self.id.0.clone()
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

    pub async fn data(&self) -> Result<Option<Vec<u8>>, AssetError> {
        if !self.data_path().exists() {
            return Ok(None);
        }
        let mut output = Vec::new();

        let mut file = File::open(self.data_path())
            .await
            .map_err(|e| self.id.io_error(e))?;
        file.read_to_end(&mut output)
            .await
            .map_err(|e| self.id.io_error(e))?;

        Ok(Some(output))
    }

    pub async fn rename(&mut self, name: String) -> Result<(), AssetError> {
        self.meta.name = Some(name);
        self.save_meta().await
    }

    pub async fn update(&mut self, data: &[u8]) -> Result<(), AssetError> {
        create_dir_all(self.id.asset_path())
            .await
            .map_err(|e| self.id.io_error(e))?;
        let mut file = File::create(self.data_path())
            .await
            .map_err(|e| self.id.io_error(e))?;
        file.write_all(data)
            .await
            .map_err(|e| self.id.io_error(e))?;
        self.meta.last_refresh = Some(OffsetDateTime::now_utc());
        self.save_meta().await
    }

    async fn save_meta(&self) -> Result<(), AssetError> {
        let meta_str = toml::to_string(&self.meta)
            .map_err(|e| AssetError::SerializeMeta(self.id.0.clone(), e))?;
        let mut file = File::create(self.id.meta_path())
            .await
            .map_err(|e| self.id.io_error(e))?;
        file.write_all(meta_str.as_bytes())
            .await
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
    #[serde(with = "time_toml_bridge")]
    last_refresh: Option<OffsetDateTime>,
    name: Option<String>,
}

mod time_toml_bridge {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use time::{Month, OffsetDateTime};
    use toml::value::{Date, Datetime, Offset, Time};

    pub fn serialize<S>(datetime: &Option<OffsetDateTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let datetime = match datetime {
            Some(datetime) => datetime,
            _ => return serializer.serialize_none(),
        };
        let date = datetime.date();
        let time = datetime.time();
        let offset = datetime.offset();
        Some(Datetime {
            date: Some(Date {
                year: date.year() as u16,
                month: u8::from(date.month()),
                day: date.day(),
            }),
            time: Some(Time {
                hour: time.hour(),
                minute: time.minute(),
                second: time.second(),
                nanosecond: time.nanosecond(),
            }),
            offset: Some(if offset.is_utc() {
                Offset::Z
            } else {
                Offset::Custom {
                    hours: offset.whole_hours(),
                    minutes: offset.minutes_past_hour().abs() as u8,
                }
            }),
        })
        .serialize(serializer)
    }
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<OffsetDateTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let datetime = Option::<Datetime>::deserialize(deserializer)?;

        let datetime = match datetime {
            Some(datetime) => datetime,
            None => return Ok(None),
        };

        let mut result = OffsetDateTime::UNIX_EPOCH;
        if let Some(date) = datetime.date {
            result = result.replace_date(
                time::Date::from_calendar_date(
                    date.year as i32,
                    Month::try_from(date.month).map_err(|e| serde::de::Error::custom(e))?,
                    date.day,
                )
                .map_err(|e| serde::de::Error::custom(e))?,
            );
        }
        if let Some(time) = datetime.time {
            result = result.replace_time(
                time::Time::from_hms_nano(time.hour, time.minute, time.second, time.nanosecond)
                    .map_err(|e| serde::de::Error::custom(e))?,
            );
        }
        if let Some(Offset::Custom { hours, minutes }) = datetime.offset {
            result = result.replace_offset(
                time::UtcOffset::from_hms(hours, minutes as i8, 0)
                    .map_err(|e| serde::de::Error::custom(e))?,
            );
        }
        Ok(Some(result))
    }
}
