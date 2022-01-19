pub use crate::data_source::{
    data::*, global_memory_status::GlobalMemoryStatusDataSource, pdh::PdhDataSource,
};

mod data;

mod global_memory_status;
mod pdh;

pub trait DataSource {
    fn name(&self) -> &'static str;

    fn update(&self) -> eyre::Result<()>;

    fn query(&mut self, query: String, preferred_format: DataFormat) -> eyre::Result<DataHandle>;
}

pub type BoxedDataSource = Box<dyn DataSource + Send + Sync>;

pub struct DataHandle(Box<dyn (Fn() -> eyre::Result<Data>) + Sync + Send>);

impl DataHandle {
    pub fn read(&self) -> eyre::Result<Data> {
        (self.0)()
    }

    pub fn read_int(&self, strict: bool) -> eyre::Result<i64> {
        self.read()
            .and_then(|data| data.unwrap_to_int(strict).map_err(Into::into))
    }

    pub fn read_float(&self, strict: bool) -> eyre::Result<f64> {
        self.read()
            .and_then(|data| data.unwrap_to_float(strict).map_err(Into::into))
    }

    pub fn read_bool(&self, strict: bool) -> eyre::Result<bool> {
        self.read()
            .and_then(|data| data.unwrap_to_bool(strict).map_err(Into::into))
    }
}
