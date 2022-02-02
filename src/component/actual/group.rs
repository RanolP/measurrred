use serde::Deserialize;
use usvg::{Node, NodeKind};

use crate::component::{Component, ComponentAction, RenderContext, SetupContext, UpdateContext};

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Group {
    #[serde(rename = "$value")]
    children: Vec<Component>,
}

impl ComponentAction for Group {
    fn setup(&mut self, context: &mut SetupContext) -> eyre::Result<()> {
        for child in self.children.iter_mut() {
            child.setup(context)?;
        }
        Ok(())
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
