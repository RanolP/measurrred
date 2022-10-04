use declarrred::rt::{Data, DataFormat};
use knowhw::Knowhw;

use crate::data_source::DataSource;

pub struct KnowhwDataSource<T: Knowhw>(pub &'static str, pub T);

impl<T: Knowhw> DataSource for KnowhwDataSource<T>
where
    T::Error: Sync + Send + 'static,
{
    fn name(&self) -> &'static str {
        self.0
    }

    fn update(&self) -> eyre::Result<()> {
        Ok(self.1.update()?)
    }

    fn query(&mut self, query: &str, preferred_format: &DataFormat) -> eyre::Result<Data> {
        Ok(self.1.query(query, preferred_format)?)
    }
}
