use std::collections::HashMap;

use crate::data_source::BoxedDataSource;

pub struct SetupContext {
    pub data_source: HashMap<&'static str, BoxedDataSource>,
}

impl SetupContext {
    pub fn find_data_source(&mut self, name: impl AsRef<str>) -> Option<&mut BoxedDataSource> {
        self.data_source.get_mut(name.as_ref())
    }
}

pub trait ComponentSetup {
    fn setup(&mut self, context: &mut SetupContext) -> eyre::Result<()>;
}
