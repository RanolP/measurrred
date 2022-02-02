pub use crate::data_source::{
    global_memory_status::GlobalMemoryStatusDataSource, pdh::PdhDataSource,
};
use crate::system::{DataFormat, DataHandle};

mod global_memory_status;
mod pdh;

pub trait DataSource {
    fn name(&self) -> &'static str;

    fn update(&self) -> eyre::Result<()>;

    fn query(&mut self, query: String, preferred_format: DataFormat) -> eyre::Result<DataHandle>;
}

pub type BoxedDataSource = Box<dyn DataSource + Send + Sync>;
