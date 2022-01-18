use std::rc::Rc;

use once_cell::sync::OnceCell;
use serde::Deserialize;
use usvg::{
    fontdb::{Family, Query},
    Group, Node, NodeExt, NodeKind, Path, PathData, Rect, Tree,
};

use super::{ComponentRender, RenderContext};

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Text {
    pub color: Option<String>,
    pub font_size: Option<f64>,
    #[serde(rename = "$value")]
    pub content: String,
}

impl ComponentRender for Text {
    fn render(&self, context: RenderContext) -> eyre::Result<Node> {
        // resvg lacks dominant-baseline support ;(
        let font_id = context
            .usvg_options
            .fontdb
            .query(&Query {
                families: &[Family::Name(&context.config.font_family)],
                ..Default::default()
            })
            .unwrap();
        let (height, ascender) = context
            .usvg_options
            .fontdb
            .with_face_data(font_id, |data, face_index| -> eyre::Result<_> {
                let font = ttf_parser::Face::from_slice(data, face_index)?;
                let scale = self.font_size.unwrap_or(16.0) / font.units_per_em() as f64;
                Ok((scale * font.height() as f64, scale * font.ascender() as f64))
            })
            .unwrap()?;

        let svg = format!(
            r#"
                <svg version="1.1" width="100" height="{font_size}" xmlns="http://www.w3.org/2000/svg">
                    <text
                        id="root"
                        dy="{dy}"
                        fill="{color}"
                        font-size="{font_size}"
                        font-family="{font_family}"
                    >
                        {content}
                    </text>
                </svg>
                "#,
            dy = ascender,
            content = self.content,
            color = self
                .color
                .as_ref()
                .unwrap_or(&context.config.foreground_color),
            font_size = self.font_size.as_ref().unwrap_or(&16.0),
            font_family = context.config.font_family,
        );
        let tree = Tree::from_str(&svg, &context.usvg_options.to_ref())?;
        let node = tree.node_by_id("root").unwrap();

        let rect = Node::new(NodeKind::Path({
            let mut path = Path::default();
            path.data = Rc::new(PathData::from_rect(
                Rect::new(0.0, 0.0, node.calculate_bbox().unwrap().width(), height).unwrap(),
            ));
            path
        }));

        let mut group = Node::new(NodeKind::Group(Group::default()));
        group.append(rect);
        group.append(node);

        Ok(group)
    }
}
