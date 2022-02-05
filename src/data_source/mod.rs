pub use self::windows::*;
use crate::system::{DataFormat, DataHandle};

mod windows;

pub trait DataSource {
    fn name(&self) -> &'static str;

    fn update(&self) -> eyre::Result<()>;

    fn query(&mut self, query: String, preferred_format: DataFormat) -> eyre::Result<DataHandle>;
}

pub type BoxedDataSource = Box<dyn DataSource + Send + Sync>;
