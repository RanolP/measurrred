use serde::{de::DeserializeOwned, Deserialize};

use crate::{
    component::RenderContext,
    system::{Data, DataFormat},
};

#[derive(Deserialize)]
#[serde(rename = "variable")]
#[serde(rename_all = "kebab-case")]
pub struct Variable {
    name: String,

    #[serde(default)]
    suffix: String,

    #[serde(default = "default_precision")]
    precision: usize,
    #[serde(default = "default_divide_by")]
    divide_by: f64,
    format: DataFormat,
}

fn default_precision() -> usize {
    2
}

fn default_divide_by() -> f64 {
    1.0
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
                format!("{}", data.as_int().ok()? / self.divide_by as i64)
            }
            DataFormat::F64 | DataFormat::Float => format!(
                "{:.precision$}",
                data.as_float().ok()? / self.divide_by,
                precision = self.precision,
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
