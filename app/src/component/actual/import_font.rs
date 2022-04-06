use std::pin::Pin;

use serde::Deserialize;
use tracing::info;
use url::Url;

use crate::component::job::{Job, WaitCompletion};
use crate::util::http;

use crate::component::{ComponentAction, SetupContext};

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ImportFont {
    url: Url,
}

impl ComponentAction for ImportFont {
    fn setup<'a>(&'a mut self) -> eyre::Result<Vec<Pin<Box<dyn Job + 'a>>>> {
        info!("Try loading {}...", self.url);
        match self.url.scheme() {
            "http" | "https" => {
                let data = http::get(&self.url)
                    .map_err(|_| eyre::eyre!("Failed to request {}", self.url))?;

                Ok(vec![WaitCompletion::new(
                    format!("Loaded {}!", self.url),
                    move |context| {
                        context.usvg_options.fontdb.load_font_data(data);
                        Ok(())
                    },
                )])
            }
            "file" => {
                let data = std::fs::read(
                    self.url
                        .to_file_path()
                        .map_err(|_| eyre::eyre!("Failed to convert {} into path.", self.url))?,
                )?;

                Ok(vec![WaitCompletion::new(
                    format!("Loaded {}!", self.url),
                    move |context| {
                        context.usvg_options.fontdb.load_font_data(data);
                        Ok(())
                    },
                )])
            }
            scheme => eyre::bail!("Unsupported url scheme: {}", scheme),
        }
    }
}
