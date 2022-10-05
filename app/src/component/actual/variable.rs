use declarrred::rt::{Data, DataFormat};
use serde::{de::DeserializeOwned, Deserialize};

use crate::{component::RenderContext, util::serde::FromStrT};
// We need FromStrT<T> because of the limitation of serde, we only can receive
// String-like types via EitherVariable<T>, So we should treat the string to fit
// our requirements.

#[derive(Deserialize)]
#[serde(rename = "variable")]
#[serde(rename_all = "kebab-case")]
pub struct Variable {
    name: String,

    #[serde(default)]
    suffix: String,

    #[serde(default = "default_precision")]
    precision: FromStrT<usize>,
    #[serde(default = "default_divide_by")]
    divide_by: FromStrT<f64>,
    format: DataFormat,
}

fn default_precision() -> FromStrT<usize> {
    FromStrT(2)
}

fn default_divide_by() -> FromStrT<f64> {
    FromStrT(1.0)
}

impl Variable {
    pub fn raw(&self, context: &RenderContext) -> Option<Data> {
        context.variables.get(&self.name).cloned()
    }

    pub fn format(&self, context: &RenderContext) -> Option<String> {
        let data = context.variables.get(&self.name)?;

        let content = match self.format {
            DataFormat::String => data.as_string().ok()?.to_string(),
            DataFormat::I32 | DataFormat::I64 | DataFormat::Int => {
                format!("{}", data.as_i64().ok()? / self.divide_by.0 as i64)
            }
            DataFormat::U32 | DataFormat::U64 | DataFormat::UInt => {
                format!("{}", data.as_u64().ok()? / self.divide_by.0 as u64)
            }
            DataFormat::F64 | DataFormat::Float => format!(
                "{:.precision$}",
                data.as_float().ok()? / self.divide_by.0,
                precision = self.precision.0,
            ),
            DataFormat::Bool => format!("{}", data.as_bool().ok()?),
        };

        Some(format!("{}{}", content, self.suffix))
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum EitherVariable<T>
where
    T: DeserializeOwned,
{
    Variable(Variable),
    #[serde(deserialize_with = "T::deserialize")]
    T(T),
}

impl EitherVariable<String> {
    pub fn format(&self, context: &RenderContext) -> Option<String> {
        match self {
            EitherVariable::Variable(v) => v.format(context),
            EitherVariable::T(t) => Some(t.clone()),
        }
    }
}
