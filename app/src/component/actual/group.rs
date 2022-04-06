use std::pin::Pin;

use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use serde::Deserialize;
use usvg::{Node, NodeKind};

use crate::component::{
    job::Job, Component, ComponentAction, RenderContext, SetupContext, UpdateContext,
};

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Group {
    #[serde(rename = "$value")]
    children: Vec<Component>,
}

impl ComponentAction for Group {
    fn setup<'a>(&'a mut self) -> eyre::Result<Vec<Pin<Box<dyn Job + 'a>>>> {
        let setup_functions = self
            .children
            .par_iter_mut()
            .map(|child| child.setup())
            .collect::<eyre::Result<Vec<_>>>()?;
        Ok(setup_functions.into_iter().flatten().collect())
    }

    fn update(&mut self, context: &mut UpdateContext) -> eyre::Result<()> {
        for child in self.children.iter_mut() {
            child.update(context)?;
        }
        Ok(())
    }

    fn render(&mut self, context: &RenderContext) -> eyre::Result<Node> {
        let mut result = Node::new(NodeKind::Group(usvg::Group::default()));
        for child in self.children.iter_mut() {
            let child_node = child.render(context)?;
            result.append(child_node);
        }
        Ok(result)
    }
}
