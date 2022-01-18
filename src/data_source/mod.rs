pub use pdh::*;
pub use global_memory_status::*;
use serde::Deserialize;

mod pdh;
mod global_memory_status;

pub trait DataSource {
    fn name(&self) -> String;

    fn update(&self) -> eyre::Result<()>;

    fn query(
        &mut self,
        query: String,
        preferred_format: PreferredDataFormat,
    ) -> eyre::Result<DataHandle>;
}

pub struct DataHandle(Box<dyn (Fn() -> eyre::Result<Data>) + Sync + Send>);

impl DataHandle {
    pub fn read(&self) -> eyre::Result<Data> {
        (self.0)()
    }

    pub fn read_int(&self, strict: bool) -> eyre::Result<i64> {
        Ok(match self.read()? {
            Data::Int(v) => v,
            Data::Float(v) => {
                if strict {
                    eyre::bail!("Expected Int but Float received")
                } else {
                    v.trunc() as i64
                }
            }
            Data::Boolean(v) => {
                if strict {
                    eyre::bail!("Expected Int but Boolean received")
                } else {
                    v as i64
                }
            }
            Data::Unknown => {
                if strict {
                    eyre::bail!("Expected Int but Unknown received")
                } else {
                    0
                }
            }
        })
    }

    pub fn read_float(&self, strict: bool) -> eyre::Result<f64> {
        Ok(match self.read()? {
            Data::Int(v) => {
                if strict {
                    eyre::bail!("Expected Float but Int received")
                } else {
                    v as f64
                }
            }
            Data::Float(v) => v,
            Data::Boolean(v) => {
                if strict {
                    eyre::bail!("Expected Float but Boolean received")
                } else {
                    v as i64 as f64
                }
            }
            Data::Unknown => {
                if strict {
                    eyre::bail!("Expected Float but Unknown received")
                } else {
                    0.0
                }
            }
        })
    }

    pub fn read_bool(&self, strict: bool) -> eyre::Result<bool> {
        Ok(match self.read()? {
            Data::Int(v) => {
                if strict {
                    eyre::bail!("Expected Bool but Int received")
                } else {
                    v != 0
                }
            }
            Data::Float(v) => {
                if strict {
                    eyre::bail!("Expected Bool but Float received")
                } else {
                    v.abs() < f64::EPSILON
                }
            }
            Data::Boolean(v) => v,
            Data::Unknown => {
                if strict {
                    eyre::bail!("Expected Bool but Unknown received")
                } else {
                    false
                }
            }
        })
    }
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PreferredDataFormat {
    Int,
    Float,
    Boolean,
}

pub enum Data {
    Int(i64),
    Float(f64),
    Boolean(bool),
    Unknown,
}
