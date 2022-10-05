use serde::Deserialize;

#[derive(Clone, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum DataFormat {
    String,
    I32,
    U32,
    I64,
    U64,
    Int,
    UInt,
    F64,
    Float,
    Bool,
}
