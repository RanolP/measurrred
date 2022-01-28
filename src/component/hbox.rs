use std::rc::Rc;

use serde::Deserialize;
use usvg::{Group, Node, NodeExt, NodeKind, Path, PathData, Rect, Transform};

use crate::system::VerticalAlignment;

use super::{Component, ComponentRender, ComponentSetup, RenderContext, SetupContext};

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct HBox {
    y_align: Option<VerticalAlignment>,

    #[serde(rename = "$value")]
    children: Vec<Component>,
}

impl ComponentSetup for HBox {
    fn setup(&mut self, context: &mut SetupContext) -> eyre::Result<()> {
        for child in self.children.iter_mut() {
            child.setup(context)?;
        }
        Ok(())
    }
}

impl ComponentRender for HBox {
    fn render(&mut self, context: &RenderContext) -> eyre::Result<Node> {
        let mut last_x_mod = 0.0;
        let mut x = 0.0;
        let mut container_height = 0.0;
        let mut nodes = Vec::new();
        let mut result = Node::new(NodeKind::Group(Group::default()));
        for child in self.children.iter_mut() {
            match child {
                Component::Margin { size } => {
                    x += *size;
                    last_x_mod = *size;
                }
                Component::SetPosition { to } => {
                    x = *to;
                    last_x_mod = 0.0;
                }
                Component::Overlap { child } => {
                    let child_node = child.render(context)?;
                    let bbox = child_node.calculate_bbox().unwrap();

                    nodes.push((x - last_x_mod, child_node));

                    x += f64::max(bbox.width() - last_x_mod, 0.0);
                    container_height = f64::max(container_height, bbox.height());

                    last_x_mod = f64::max(bbox.width(), last_x_mod);
                }
                _ => {
                    let child_node = child.render(context)?;
                    let bbox = child_node.calculate_bbox().unwrap();

                    nodes.push((x, child_node));

                    x += bbox.width();
                    container_height = f64::max(container_height, bbox.height());

                    last_x_mod = bbox.width();
                }
            }
        }

        result.append(Node::new(NodeKind::Path({
            let mut path = Path::default();
            path.data = Rc::new(PathData::from_rect(
                Rect::new(0.0, 0.0, x, container_height).unwrap(),
            ));
            path
        })));

        for (x, node) in nodes {
            let bbox = node.calculate_bbox().unwrap();

            let mut child_transformer = Node::new(NodeKind::Group({
                let mut group = Group::default();
                group.transform = Transform::new_translate(
                    x,
                    self.y_align
                        .as_ref()
                        .unwrap_or(&VerticalAlignment::Top)
                        .align(container_height, bbox.height()),
                );
                group
            }));
            child_transformer.append(node);

            result.append(child_transformer);
        }

        Ok(result)
    }
}
