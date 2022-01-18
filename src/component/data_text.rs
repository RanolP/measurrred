use serde::Deserialize;

use crate::{
    component::Text,
    data_source::{DataHandle, PreferredDataFormat},
};

use super::{ComponentRender, ComponentSetup, RenderContext, SetupContext};

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DataText {
    color: Option<String>,
    font_size: Option<f64>,

    precision: Option<usize>,
    divide_by: Option<f64>,

    source: String,
    query: String,
    format: PreferredDataFormat,

    #[serde(skip)]
    handle: Option<DataHandle>,
}

impl ComponentSetup for DataText {
    fn setup(&mut self, context: &mut SetupContext) -> eyre::Result<()> {
        self.handle = Some(
            context
                .data_source
                .get_mut(&self.source)
                .unwrap()
                .query(self.query.clone(), self.format.clone())?,
        );
        Ok(())
    }
}

impl ComponentRender for DataText {
    fn render(&self, context: RenderContext) -> eyre::Result<usvg::Node> {
        let divide_by = self.divide_by.unwrap_or(1.0);
        let handle = self.handle.as_ref().unwrap();
        let content = match self.format {
            PreferredDataFormat::Int => format!("{}", handle.read_int(false)? / (divide_by as i64)),
            PreferredDataFormat::Float => format!(
                "{:.precision$}",
                handle.read_float(false)? / divide_by,
                precision = self.precision.unwrap_or(2)
            ),
            PreferredDataFormat::Boolean => format!("{}", handle.read_bool(false)?),
        };
        let text = Text {
            color: self.color.clone(),
            font_size: self.font_size.clone(),
            content,
        };
        text.render(context)
    }
}
