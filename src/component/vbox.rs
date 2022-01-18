use std::rc::Rc;

use serde::Deserialize;
use usvg::{Group, Node, NodeExt, NodeKind, Path, PathData, Rect, Transform};

use crate::system::HorizontalAlignment;

use super::{Component, ComponentRender, ComponentSetup, RenderContext, SetupContext};

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct VBox {
    x_align: Option<HorizontalAlignment>,

    #[serde(rename = "$value")]
    children: Vec<Component>,
}

impl ComponentSetup for VBox {
    fn setup(&mut self, context: &mut SetupContext) -> eyre::Result<()> {
        for child in self.children.iter_mut() {
            child.setup(context)?;
        }
        Ok(())
    }
}

impl ComponentRender for VBox {
    fn render(&self, render_context: RenderContext) -> eyre::Result<Node> {
        let mut y = 0.0;
        let mut container_width = 0.0;
        let mut nodes = Vec::new();
        let mut result = Node::new(NodeKind::Group(Group::default()));
        for child in &self.children {
            match child {
                &Component::Margin { size } => {
                    y += size;
                    continue;
                }
                &Component::SetPosition { to } => {
                    y -= to;
                    continue;
                }
                _ => {}
            }
            let child_node = child.render(render_context.clone())?;
            let bbox = child_node.calculate_bbox().unwrap();

            nodes.push((y, child_node));

            y += bbox.height();
            container_width = f64::max(container_width, bbox.width());
        }

        result.append(Node::new(NodeKind::Path({
            let mut path = Path::default();
            path.data = Rc::new(PathData::from_rect(
                Rect::new(0.0, 0.0, container_width, y).unwrap(),
            ));
            path
        })));

        for (y, node) in nodes {
            let bbox = node.calculate_bbox().unwrap();

            let mut child_transformer = Node::new(NodeKind::Group({
                let mut group = Group::default();
                group.transform = Transform::new_translate(
                    self.x_align
                        .as_ref()
                        .unwrap_or(&HorizontalAlignment::Left)
                        .align(container_width, bbox.width()),
                    y,
                );
                group
            }));
            child_transformer.append(node);

            result.append(child_transformer);
        }

        Ok(result)
    }
}
