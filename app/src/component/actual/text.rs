use std::rc::Rc;

use serde::Deserialize;
use tracing_unwrap::OptionExt;
use usvg::{
    fontdb::{Family, Query},
    Group, Node, NodeExt, NodeKind, Path, PathData, Rect, Tree,
};

use crate::component::{ComponentAction, RenderContext};

use super::EitherVariable;

#[derive(Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TextAlign {
    Left,
    Center,
    Right,
}

impl Default for TextAlign {
    fn default() -> Self {
        TextAlign::Left
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Text {
    pub color: Option<String>,

    #[serde(default)]
    pub text_align: TextAlign,

    pub font_size: Option<f64>,
    pub font_family: Option<String>,
    pub font_weight: Option<String>,

    #[serde(rename = "$value")]
    pub content: Vec<EitherVariable<String>>,
}

impl ComponentAction for Text {
    fn render(&mut self, context: &RenderContext) -> eyre::Result<Node> {
        // resvg lacks dominant-baseline support ;(
        let font_size = self.font_size.unwrap_or(16.0);
        let font_family = self
            .font_family
            .as_ref()
            .unwrap_or(&context.config.general.font_family);

        let font_id = match context.usvg_options.fontdb.query(&Query {
            families: &[Family::Name(font_family)],
            ..Default::default()
        }) {
            Some(font_id) => font_id,
            None => {
                eyre::bail!("Failed to find font {}{}", &font_family, {
                    let mut families = context
                        .usvg_options
                        .fontdb
                        .faces()
                        .iter()
                        .map(|face| face.family.clone())
                        .collect::<std::collections::HashSet<_>>()
                        .into_iter()
                        .collect::<Vec<_>>();
                    families
                        .sort_by_key(|family| strsim::damerau_levenshtein(&font_family, &family));
                    if families.len() > 0 {
                        format!(
                            ", you may wanted to use one of these fonts: {}",
                            families[0..5].join(", ")
                        )
                    } else {
                        ".".to_string()
                    }
                });
            }
        };
        let (height, ascender) = context
            .usvg_options
            .fontdb
            .with_face_data(font_id, |data, face_index| -> eyre::Result<_> {
                let font = ttf_parser::Face::from_slice(data, face_index)?;
                let scale = font_size / font.units_per_em() as f64;
                Ok((scale * font.height() as f64, scale * font.ascender() as f64))
            })
            .unwrap_or_log()?;

        let svg = format!(
            r#"
                <svg version="1.1" width="100" height="{font_size}" xmlns="http://www.w3.org/2000/svg">
                    <text
                        id="root"
                        dy="{dy}"
                        fill="{color}"
                        font-size="{font_size}"
                        font-family="{font_family}"
                        text-anchor="{text_anchor}"
                        {font_weight}
                    >
                        {content}
                    </text>
                </svg>
                "#,
            dy = ascender,
            content = self
                .content
                .iter()
                .flat_map(|fragment| fragment.format(context))
                .collect::<Vec<_>>()
                .join(""),
            color = self
                .color
                .as_ref()
                .unwrap_or(&context.config.general.foreground_color.to_string()),
            font_size = font_size,
            font_family = font_family,
            text_anchor = match self.text_align {
                TextAlign::Left => "start",
                TextAlign::Center => "middle",
                TextAlign::Right => "end",
            },
            font_weight = self
                .font_weight
                .as_ref()
                .or(context.config.general.font_weight.as_ref())
                .map(|weight| format!(r#"font-weight="{}""#, weight))
                .unwrap_or_default()
        );
        let tree = Tree::from_str(&svg, &context.usvg_options.to_ref())?;
        let node = tree.node_by_id("root").unwrap();

        let width = match &*node.borrow() {
            NodeKind::Path(path) => path.text_bbox.unwrap().width(),
            _ => node.calculate_bbox().unwrap().width(),
        };

        let rect = Node::new(NodeKind::Path(Path {
            data: Rc::new(PathData::from_rect(
                Rect::new(
                    match self.text_align {
                        TextAlign::Left => 0.0,
                        TextAlign::Center => -width / 2.0,
                        TextAlign::Right => -width,
                    },
                    0.0,
                    width,
                    height,
                )
                .unwrap_or_log(),
            )),
            ..Default::default()
        }));

        let mut group = Node::new(NodeKind::Group(Group::default()));
        group.append(rect);
        group.append(node);

        Ok(group)
    }
}
