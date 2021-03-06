use serde::Deserialize;

#[derive(Clone, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum DataFormat {
    String,
    I32,
    I64,
    Int,
    F64,
    Float,
    Bool,
}
