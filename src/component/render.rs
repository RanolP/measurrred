use usvg::{Node, Options};

use crate::config::MeasurrredConfig;

#[derive(Clone)]
pub struct RenderContext<'a> {
    pub viewbox_width: f64,
    pub viewbox_height: f64,
    pub usvg_options: &'a Options,
    pub config: &'a MeasurrredConfig,
}

pub trait ComponentRender {
    fn render(&mut self, context: RenderContext) -> eyre::Result<Node>;
}
