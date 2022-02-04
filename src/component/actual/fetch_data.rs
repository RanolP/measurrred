use serde::Deserialize;
use tracing_unwrap::OptionExt;

use crate::{
    component::{ComponentAction, SetupContext, UpdateContext},
    system::{DataFormat, DataHandle},
};

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct FetchData {
    name: String,

    source: String,
    query: String,
    format: DataFormat,

    #[serde(skip)]
    handle: Option<DataHandle>,
}

impl ComponentAction for FetchData {
    fn setup<'a>(
        &'a mut self,
    ) -> eyre::Result<Box<dyn FnOnce(&mut SetupContext) -> eyre::Result<()> + Send + 'a>> {
        Ok(Box::new(|context| {
            self.handle = Some(
                context
                    .find_data_source(&self.source)
                    .ok_or(eyre::eyre!("Unknown data source: {}", &self.source))?
                    .query(self.query.clone(), self.format.clone())?,
            );
            Ok(())
        }))
    }
    fn update(&mut self, context: &mut UpdateContext) -> eyre::Result<()> {
        let data = self.handle.as_ref().unwrap_or_log().read()?;
        context.variables.insert(self.name.clone(), data);
        Ok(())
    }
}
