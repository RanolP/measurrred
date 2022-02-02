use std::collections::HashMap;

use tiny_skia::Pixmap;
use usvg::Options;

use crate::{config::MeasurrredConfig, data_source::BoxedDataSource, system::Data};

pub struct SetupContext {
    pub data_source: HashMap<&'static str, BoxedDataSource>,
    pub usvg_options: Options,
}

impl SetupContext {
    pub fn find_data_source(&mut self, name: impl AsRef<str>) -> Option<&mut BoxedDataSource> {
        self.data_source.get_mut(name.as_ref())
    }
}

pub struct UpdateContext<'a> {
    pub config: &'a MeasurrredConfig,
    pub variables: HashMap<String, Data>,
}

impl<'a> UpdateContext<'a> {
    pub fn new(config: &'a MeasurrredConfig) -> Self {
        UpdateContext {
            config,
            variables: HashMap::new(),
        }
    }
}

pub struct RenderContext<'a> {
    pub viewbox_width: f64,
    pub viewbox_height: f64,
    pub usvg_options: &'a Options,
    pub config: &'a MeasurrredConfig,
    pub variables: HashMap<String, Data>,
}

impl<'a> RenderContext<'a> {
    pub fn new(
        viewbox: &'a Pixmap,
        usvg_options: &'a Options,
        update_context: UpdateContext<'a>,
    ) -> Self {
        RenderContext {
            viewbox_width: viewbox.width() as f64,
            viewbox_height: viewbox.height() as f64,
            usvg_options,
            config: update_context.config,
            variables: update_context.variables,
        }
    }
}

pub trait ComponentAction {
    fn setup(&mut self, context: &mut SetupContext) -> eyre::Result<()> {
        Ok(())
    }

    fn update(&mut self, context: &mut UpdateContext) -> eyre::Result<()> {
        Ok(())
    }

    fn render(&mut self, context: &RenderContext) -> eyre::Result<usvg::Node> {
        Ok(usvg::Node::new(usvg::NodeKind::Group(
            usvg::Group::default(),
        )))
    }
}
