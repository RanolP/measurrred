use std::collections::HashMap;

use crate::data_source::DataSource;

pub struct SetupContext {
    pub data_source: HashMap<String, Box<dyn DataSource + Sync + Send>>,
}

pub trait ComponentSetup {
    fn setup(&mut self, context: &mut SetupContext) -> eyre::Result<()>;
}
