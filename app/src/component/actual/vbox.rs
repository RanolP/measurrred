use std::{pin::Pin, rc::Rc};

use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use serde::Deserialize;
use usvg::{Group, Node, NodeExt, NodeKind, Path, PathData, Rect, Transform};

use crate::{
    component::{job::Job, Component, ComponentAction, RenderContext, SetupContext, UpdateContext},
    system::HorizontalAlignment,
};

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct VBox {
    x_align: Option<HorizontalAlignment>,

    #[serde(rename = "$value")]
    children: Vec<Component>,
}

impl ComponentAction for VBox {
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
        let mut last_y_mod = 0.0;
        let mut y = 0.0;
        let mut container_width = 0.0;
        let mut nodes = Vec::new();
        let mut result = Node::new(NodeKind::Group(Group::default()));
        for child in self.children.iter_mut() {
            match child {
                Component::Margin { size } => {
                    let size = size.translate_to_px(context.viewbox_width, context.viewbox_height);
                    y += size;
                    last_y_mod = size;
                }
                Component::SetPosition { to } => {
                    let to = to.translate_to_px(context.viewbox_width, context.viewbox_height);
                    y = to;
                    last_y_mod = 0.0;
                }
                Component::Overlap { child } => {
                    let child_node = child.render(context)?;
                    let bbox = child_node.calculate_bbox().unwrap();

                    nodes.push((y - last_y_mod, child_node));

                    y += f64::max(bbox.height() - last_y_mod, 0.0);
                    container_width = f64::max(container_width, bbox.width());

                    last_y_mod = f64::max(bbox.height(), last_y_mod);
                }
                _ => {
                    let child_node = child.render(context)?;
                    let bbox = child_node.calculate_bbox().unwrap();

                    nodes.push((y, child_node));

                    let dy = bbox.bottom();

                    y += dy;
                    container_width = f64::max(container_width, bbox.width());

                    last_y_mod = dy;
                }
            }
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
