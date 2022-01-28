use serde::Deserialize;
use usvg::{Node, NodeKind};

use super::{Component, ComponentRender, ComponentSetup, RenderContext, SetupContext};

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Group {
    #[serde(rename = "$value")]
    children: Vec<Component>,
}

impl ComponentSetup for Group {
    fn setup(&mut self, context: &mut SetupContext) -> eyre::Result<()> {
        for child in self.children.iter_mut() {
            child.setup(context)?;
        }
        Ok(())
    }
}

impl ComponentRender for Group {
    fn render(&mut self, context: &RenderContext) -> eyre::Result<Node> {
        let mut result = Node::new(NodeKind::Group(usvg::Group::default()));
        for child in self.children.iter_mut() {
            let child_node = child.render(context)?;
            result.append(child_node);
        }
        Ok(result)
    }
}
