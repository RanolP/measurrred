use std::pin::Pin;

use serde::Deserialize;

use crate::{
    component::{
        action::DataQueryVariable,
        job::{Job, WaitCompletion},
        ComponentAction, SetupContext,
    },
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
    fn setup(& mut self) -> eyre::Result<Vec<Pin<Box<dyn Job + 'static>>>> {
        let name = self.name.clone();
        let source = self.source.clone();
        let query = self.query.clone();
        let format = self.format.clone();
        Ok(vec![WaitCompletion::new(
            "Adding data query...",
            move |context| {
                context.data_queries.push(DataQueryVariable {
                    name,
                    source,
                    query,
                    format,
                });
                Ok(())
            },
        )])
    }
}
