pub use self::windows::*;
use crate::system::{Data, DataFormat};

mod windows;

pub trait DataSource {
    fn name(&self) -> &'static str;

    fn update(&self) -> eyre::Result<()>;

    fn query(&mut self, query: &str, preferred_format: &DataFormat) -> eyre::Result<Data>;
}

pub type BoxedDataSource = Box<dyn DataSource + Send + Sync>;
