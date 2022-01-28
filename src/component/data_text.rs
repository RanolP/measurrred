use serde::Deserialize;

use crate::{
    component::Text,
    data_source::{DataFormat, DataHandle},
};

use super::{text::TextAlign, ComponentRender, ComponentSetup, RenderContext, SetupContext};

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DataText {
    color: Option<String>,
    font_size: Option<f64>,
    font_family: Option<String>,
    font_weight: Option<String>,
    #[serde(default)]
    text_align: TextAlign,

    precision: Option<usize>,
    divide_by: Option<f64>,

    source: String,
    query: String,
    input_format: Option<DataFormat>,
    output_format: DataFormat,

    #[serde(skip)]
    handle: Option<DataHandle>,
}

impl ComponentSetup for DataText {
    fn setup(&mut self, context: &mut SetupContext) -> eyre::Result<()> {
        self.handle = Some(
            context
                .find_data_source(&self.source)
                .ok_or(eyre::eyre!("Unknown data source: {}", &self.source))?
                .query(
                    self.query.clone(),
                    self.input_format
                        .as_ref()
                        .unwrap_or(&self.output_format)
                        .clone(),
                )?,
        );
        Ok(())
    }
}

impl ComponentRender for DataText {
    fn render(&mut self, context: &RenderContext) -> eyre::Result<usvg::Node> {
        let divide_by = self.divide_by.unwrap_or(1.0);
        let handle = self.handle.as_ref().unwrap();
        let content = match self.output_format {
            DataFormat::I32 | DataFormat::I64 | DataFormat::Int => {
                format!("{}", handle.read_int(false)? / (divide_by as i64))
            }
            DataFormat::Float => format!(
                "{:.precision$}",
                handle.read_float(false)? / divide_by,
                precision = self.precision.unwrap_or(2)
            ),
            DataFormat::Boolean => format!("{}", handle.read_bool(false)?),
        };
        let mut text = Text {
            color: self.color.clone(),
            font_size: self.font_size.clone(),
            font_family: self.font_family.clone(),
            font_weight: self.font_weight.clone(),
            text_align: self.text_align.clone(),
            content,
        };
        text.render(context)
    }
}
