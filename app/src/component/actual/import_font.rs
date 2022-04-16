use async_stream::try_stream;
use serde::Deserialize;
use url::Url;

use crate::component::job::Job;
use crate::util::http;

use crate::component::{ComponentAction, JobStage};

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ImportFont {
    url: Url,
}

pub enum FontImportStage {
    Initial,
    Checking,
    Fetching,
    BytesRead(Vec<u8>),
}

impl ComponentAction for ImportFont {
    fn setup(&mut self) -> Vec<Job> {
        let url = self.url.clone();
        vec![Box::pin(try_stream! {
            yield JobStage::Progress {
                label: format!("Try loading {}", url),
                value: 0.0
            };
            let data = match url.scheme() {
                "http" | "https" => {
                    yield JobStage::Progress {
                        label: format!("Fetching {} from online...", url),
                        value: 3.0
                    };
                    http::get(&url).await
                .map_err(|_| eyre::eyre!("Failed to request {}", url))?
                }
                "file" => {
                    std::fs::read(
                        url
                            .to_file_path()
                            .map_err(|_| eyre::eyre!("Failed to convert {} into path.", url))?,
                    )?
                }
                scheme => {
                    yield JobStage::Fail {
                        label: format!("Unsupported url scheme: {}", scheme)
                    };
                    return
                },
            };
            yield JobStage::Completed {
                label: format!("Loaded {}!", url),
                finalizer: Box::new(move |context| {
                    context.usvg_options.fontdb.load_font_data(data);
                    Ok(())
                })
            }
        })]
    }
}
