use serde::Deserialize;

use crate::{
    component::{action::DataQueryVariable, ComponentAction, SetupContext},
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
    fn setup<'a>(
        &'a mut self,
    ) -> eyre::Result<Box<dyn FnOnce(&mut SetupContext) -> eyre::Result<()> + Send + 'a>> {
        Ok(Box::new(|context| {
            context.data_queries.push(DataQueryVariable {
                name: self.name.clone(),
                source: self.source.clone(),
                query: self.query.clone(),
                format: self.format.clone(),
            });
            Ok(())
        }))
    }
}
