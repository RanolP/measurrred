use serde::Deserialize;
use tracing::info;
use url::Url;

use crate::util::http;

use super::{ComponentSetup, SetupContext};

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ImportFont {
    url: Url,
}

impl ComponentSetup for ImportFont {
    fn setup(&mut self, context: &mut SetupContext) -> eyre::Result<()> {
        info!("Try loading {}...", self.url);
        match self.url.scheme() {
            "http" | "https" => {
                context.usvg_options.fontdb.load_font_data(
                    http::get(&self.url)
                        .map_err(|_| eyre::eyre!("Failed to request {}", self.url))?,
                );
            }
            "file" => {
                context.usvg_options.fontdb.load_font_file(
                    self.url
                        .to_file_path()
                        .map_err(|_| eyre::eyre!("Failed to convert {} into path.", self.url))?,
                )?;
            }
            scheme => eyre::bail!("Unsupported url scheme: {}", scheme),
        }
        Ok(())
    }
}
