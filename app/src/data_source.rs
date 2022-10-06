use declarrred::rt::{Data, DataFormat};
use knowhw::Knowhw;

pub struct KnowhwDataSource<T: Knowhw>(pub T);

impl<T: Knowhw> KnowhwDataSource<T>
where
    T: Sync + Send + 'static,
    T::Error: Sync + Send + 'static,
{
    pub fn boxed(knowhw: T) -> BoxedDataSource {
        Box::new(KnowhwDataSource(knowhw))
    }
}

impl<T: Knowhw> DataSource for KnowhwDataSource<T>
where
    T::Error: Sync + Send + 'static,
{
    fn update(&self) -> eyre::Result<()> {
        Ok(self.0.update()?)
    }

    fn query(&mut self, query: &str, preferred_format: &DataFormat) -> eyre::Result<Data> {
        Ok(self.0.query(query, preferred_format)?)
    }
}

pub trait DataSource {
    fn update(&self) -> eyre::Result<()>;

    fn query(&mut self, query: &str, preferred_format: &DataFormat) -> eyre::Result<Data>;
}

pub type BoxedDataSource = Box<dyn DataSource + Send + Sync>;
