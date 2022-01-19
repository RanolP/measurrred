use serde::Deserialize;
use thiserror::Error;

#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum DataFormat {
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

#[derive(Error, Debug)]
#[error("Failed to conver {from_format:?} to {to_format:?}")]
pub struct DataConversionError {
    from_format: Option<DataFormat>,
    to_format: DataFormat,
}

impl Data {
    pub fn unwrap_to_int(&self, strict: bool) -> Result<i64, DataConversionError> {
        match &self {
            Data::Int(v) => Ok(*v),
            Data::Float(v) => {
                if strict {
                    Err(DataConversionError {
                        from_format: Some(DataFormat::Float),
                        to_format: DataFormat::Int,
                    })
                } else {
                    Ok(v.trunc() as i64)
                }
            }
            Data::Boolean(v) => {
                if strict {
                    Err(DataConversionError {
                        from_format: Some(DataFormat::Boolean),
                        to_format: DataFormat::Int,
                    })
                } else {
                    Ok(*v as i64)
                }
            }
            Data::Unknown => {
                if strict {
                    Err(DataConversionError {
                        from_format: None,
                        to_format: DataFormat::Int,
                    })
                } else {
                    Ok(0)
                }
            }
        }
    }

    pub fn unwrap_to_float(&self, strict: bool) -> Result<f64, DataConversionError> {
        match &self {
            Data::Int(v) => {
                if strict {
                    Err(DataConversionError {
                        from_format: Some(DataFormat::Int),
                        to_format: DataFormat::Float,
                    })
                } else {
                    Ok(*v as f64)
                }
            }
            Data::Float(v) => Ok(*v),
            Data::Boolean(v) => {
                if strict {
                    Err(DataConversionError {
                        from_format: Some(DataFormat::Boolean),
                        to_format: DataFormat::Float,
                    })
                } else {
                    Ok(*v as i64 as f64)
                }
            }
            Data::Unknown => {
                if strict {
                    Err(DataConversionError {
                        from_format: None,
                        to_format: DataFormat::Float,
                    })
                } else {
                    Ok(0.0)
                }
            }
        }
    }

    pub fn unwrap_to_bool(&self, strict: bool) -> Result<bool, DataConversionError> {
        match &self {
            Data::Int(v) => {
                if strict {
                    Err(DataConversionError {
                        from_format: Some(DataFormat::Int),
                        to_format: DataFormat::Boolean,
                    })
                } else {
                    Ok(*v != 0)
                }
            }
            Data::Float(v) => {
                if strict {
                    Err(DataConversionError {
                        from_format: Some(DataFormat::Float),
                        to_format: DataFormat::Boolean,
                    })
                } else {
                    Ok(v.abs() > f64::EPSILON)
                }
            }
            Data::Boolean(v) => Ok(*v),
            Data::Unknown => {
                if strict {
                    Err(DataConversionError {
                        from_format: None,
                        to_format: DataFormat::Boolean,
                    })
                } else {
                    Ok(false)
                }
            }
        }
    }
}
