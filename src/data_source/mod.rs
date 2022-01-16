pub use pdh::*;

mod pdh;

pub trait DataSource {
    fn name(&self) -> String;

    fn update(&mut self) -> eyre::Result<()>;

    fn query(&mut self, query: String, preferred_format: PreferredDataType) -> eyre::Result<Data>;

    fn query_int(&mut self, query: String, strict: bool) -> eyre::Result<i64> {
        Ok(match self.query(query, PreferredDataType::Int)? {
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

    fn query_float(&mut self, query: String, strict: bool) -> eyre::Result<f64> {
        Ok(match self.query(query, PreferredDataType::Float)? {
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

    fn query_bool(&mut self, query: String, strict: bool) -> eyre::Result<bool> {
        Ok(match self.query(query, PreferredDataType::Boolean)? {
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

pub enum PreferredDataType {
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
