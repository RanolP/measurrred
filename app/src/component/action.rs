use std::{collections::HashMap, pin::Pin};

use usvg::Options;

use crate::{
    config::MeasurrredConfig,
    system::{Data, DataFormat},
};

use super::job::Job;

pub struct SetupContext {
    pub usvg_options: Options,
    pub data_queries: Vec<DataQueryVariable>,
}

pub struct DataQueryVariable {
    pub name: String,
    pub source: String,
    pub query: String,
    pub format: DataFormat,
}

impl SetupContext {
    pub fn new(usvg_options: Options) -> Self {
        SetupContext {
            usvg_options,
            data_queries: Vec::new(),
        }
    }
}

pub struct UpdateContext<'a> {
    pub config: &'a MeasurrredConfig,
}

impl<'a> UpdateContext<'a> {
    pub fn new(config: &'a MeasurrredConfig) -> Self {
        UpdateContext { config }
    }
}

pub struct RenderContext<'a> {
    pub viewbox_width: f64,
    pub viewbox_height: f64,
    pub usvg_options: &'a Options,
    pub config: &'a MeasurrredConfig,
    pub variables: &'a HashMap<String, Data>,
}

impl<'a> RenderContext<'a> {
    pub fn new(
        viewbox_width: f64,
        viewbox_height: f64,
        usvg_options: &'a Options,
        config: &'a MeasurrredConfig,
        variables: &'a HashMap<String, Data>,
    ) -> Self {
        RenderContext {
            viewbox_width,
            viewbox_height,
            usvg_options,
            config,
            variables,
        }
    }
}

pub trait ComponentAction {
    fn setup<'a>(
        &'a mut self,
    ) -> eyre::Result<Vec<Pin<Box<dyn Job + 'a>>>> {
        Ok(Vec::new())
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
