use std::{collections::LinkedList, rc::Rc};

use declarrred::rt::Data;
use serde::Deserialize;
use tracing_unwrap::OptionExt;
use usvg::{
    Fill, Group, Node, NodeKind, Opacity, Paint, Path, PathData, Rect, Stroke, StrokeWidth,
};

use crate::{
    component::{job::Job, ComponentAction, RenderContext},
    system::{Color, Length},
};

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Graph {
    width: Length,
    height: Length,

    min: f64,
    max: f64,
    #[serde(default = "default_sample_count")]
    sample_count: usize,

    stroke_color: Color,
    stroke_width: f64,

    fill_color: Option<Color>,
    #[serde(default = "default_fill_opacity")]
    fill_opacity: f64,

    name: String,

    #[serde(skip)]
    samples: LinkedList<f64>,
}

const fn default_sample_count() -> usize {
    10
}

const fn default_fill_opacity() -> f64 {
    0.6
}

impl ComponentAction for Graph {
    fn setup<'a>(&'a mut self) -> Vec<Job> {
        self.samples = LinkedList::from_iter(vec![f64::NAN; self.sample_count].into_iter());
        Vec::new()
    }

    fn render(&mut self, context: &RenderContext) -> eyre::Result<usvg::Node> {
        let width_px = self
            .width
            .translate_to_px(context.viewbox_width, context.viewbox_height);
        let height_px = self
            .height
            .translate_to_px(context.viewbox_width, context.viewbox_height);

        let data = context
            .variables
            .get(&self.name)
            .unwrap_or(&Data::Unknown)
            .as_float()?;
        self.samples.pop_front();
        self.samples.push_back(data);

        let mut line = PathData::new();
        let mut fill = PathData::new();
        let mut first_point = None;

        for (i, sample) in self.samples.iter().enumerate() {
            if sample.is_nan() {
                continue;
            }

            let x = i as f64 * width_px / (self.sample_count - 1) as f64;
            let y = height_px - (sample - self.min) / (self.max - self.min) * height_px;

            if first_point.is_none() {
                line.push_move_to(x, y);
                fill.push_move_to(x, height_px);
                fill.push_line_to(x, y);
                first_point = Some((x, y));
            } else {
                line.push_line_to(x, y);
                fill.push_line_to(x, y);
            }
        }

        fill.push_line_to(width_px, height_px);
        if let Some((x, _)) = first_point {
            fill.push_line_to(x, height_px);
        }

        let mut group = Node::new(NodeKind::Group(Group::default()));

        group.append(Node::new(NodeKind::Path(Path {
            data: Rc::new(PathData::from_rect(
                Rect::new(0.0, 0.0, width_px, height_px).unwrap_or_log(),
            )),
            ..Default::default()
        })));

        group.append(Node::new(NodeKind::Path(Path {
            data: Rc::new(line),
            stroke: Some(Stroke {
                paint: Paint::Color(self.stroke_color.to_usvg_color()),
                width: StrokeWidth::new(self.stroke_width),
                ..Default::default()
            }),
            ..Default::default()
        })));

        let fill_color = self.fill_color.as_ref().unwrap_or(&self.stroke_color);

        group.append(Node::new(NodeKind::Path(Path {
            data: Rc::new(fill),
            fill: Some(Fill {
                paint: Paint::Color(fill_color.to_usvg_color()),
                opacity: Opacity::new(self.fill_opacity),
                ..Default::default()
            }),
            ..Default::default()
        })));

        Ok(group)
    }
}
