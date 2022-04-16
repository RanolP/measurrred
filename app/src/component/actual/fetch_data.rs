use async_stream::try_stream;
use serde::Deserialize;

use crate::{
    component::{action::DataQueryVariable, job::Job, ComponentAction, JobStage},
    system::DataFormat,
};

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct FetchData {
    name: String,

    source: String,
    query: String,
    format: DataFormat,
}

impl ComponentAction for FetchData {
    fn setup(&mut self) -> Vec<Job> {
        let name = self.name.clone();
        let source = self.source.clone();
        let query = self.query.clone();
        let format = self.format.clone();
        vec![Box::pin(try_stream! {
            yield JobStage::Completed {
                label: "Adding data query...".to_string(),
                finalizer: Box::new(move |context| {
                    context.data_queries.push(DataQueryVariable {
                        name,
                        source,
                        query,
                        format,
                    });
                    Ok(())
                })
            }
        })]
    }
}
