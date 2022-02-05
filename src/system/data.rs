use std::borrow::Cow;

use thiserror::Error;

use super::DataFormat;

#[derive(Clone, Debug)]
pub enum Data {
    String(String),
    I32(i32),
    I64(i64),
    F64(f64),
    Bool(bool),
    Unknown,
}

#[derive(Error, Debug)]
#[error(
    "Failed to convert{:?} to {to_format:?}",
    "from_format.map(|v| format!(\" {v:?}\")).unwrap_or(\"\".to_string())"
)]
pub struct DataConversionError {
    from_format: Option<DataFormat>,
    to_format: DataFormat,
}

impl Data {
    pub fn as_string(&self) -> Result<Cow<str>, DataConversionError> {
        Ok(match self {
            Data::String(v) => Cow::Borrowed(&v),
            Data::I32(v) => Cow::Owned(v.to_string()),
            Data::I64(v) => Cow::Owned(v.to_string()),
            Data::F64(v) => Cow::Owned(v.to_string()),
            Data::Bool(v) => Cow::Owned(v.to_string()),
            Data::Unknown => Cow::Borrowed(""),
        })
    }

    pub fn as_int(&self) -> Result<i64, DataConversionError> {
        match self {
            Data::String(v) => v.parse().map_err(|_| DataConversionError {
                from_format: Some(DataFormat::String),
                to_format: DataFormat::Int,
            }),
            Data::I32(v) => Ok(*v as i64),
            Data::I64(v) => Ok(*v),
            Data::F64(v) => Ok(*v as i64),
            Data::Bool(v) => Ok(*v as i64),
            Data::Unknown => Ok(0),
        }
    }

    pub fn as_float(&self) -> Result<f64, DataConversionError> {
        match self {
            Data::String(v) => v.parse().map_err(|_| DataConversionError {
                from_format: Some(DataFormat::String),
                to_format: DataFormat::Float,
            }),
            Data::I32(v) => Ok(*v as f64),
            Data::I64(v) => Ok(*v as f64),
            Data::F64(v) => Ok(*v),
            Data::Bool(v) => Ok(*v as i64 as f64),
            Data::Unknown => Ok(0.0),
        }
    }

    pub fn as_bool(&self) -> Result<bool, DataConversionError> {
        match self {
            Data::String(v) => v.parse().map_err(|_| DataConversionError {
                from_format: Some(DataFormat::String),
                to_format: DataFormat::Bool,
            }),
            Data::I32(v) => Ok(*v != 0),
            Data::I64(v) => Ok(*v != 0),
            Data::F64(v) => Ok(v.abs() > f64::EPSILON),
            Data::Bool(v) => Ok(*v),
            Data::Unknown => Ok(false),
        }
    }
}
