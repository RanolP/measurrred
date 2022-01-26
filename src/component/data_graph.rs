use std::{collections::LinkedList, rc::Rc, str::FromStr};

use serde::Deserialize;
use tracing_unwrap::{OptionExt, ResultExt};
use usvg::{
    Fill, Group, Node, NodeKind, Opacity, Paint, Path, PathData, Rect, Stroke, StrokeWidth,
};

use crate::{
    data_source::{DataFormat, DataHandle},
    system::Color,
};

use super::{ComponentRender, ComponentSetup, RenderContext, SetupContext};

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DataGraph {
    width: f64,
    height: f64,

    min: f64,
    max: f64,
    #[serde(default = "default_sample_count")]
    sample_count: usize,

    stroke_color: Color,
    stroke_width: f64,

    fill_color: Option<Color>,
    #[serde(default = "default_fill_opacity")]
    fill_opacity: f64,

    source: String,
    query: String,
    input_format: Option<DataFormat>,

    #[serde(skip)]
    handle: Option<DataHandle>,
    #[serde(skip)]
    samples: LinkedList<f64>,
}

const fn default_sample_count() -> usize {
    10
}

const fn default_fill_opacity() -> f64 {
    0.6
}

impl ComponentSetup for DataGraph {
    fn setup(&mut self, context: &mut SetupContext) -> eyre::Result<()> {
        self.handle = Some(
            context
                .find_data_source(&self.source)
                .ok_or(eyre::eyre!("Unknown data source: {}", &self.source))?
                .query(
                    self.query.clone(),
                    self.input_format
                        .as_ref()
                        .unwrap_or(&DataFormat::Float)
                        .clone(),
                )?,
        );
        self.samples = LinkedList::from_iter(vec![f64::NAN; self.sample_count].into_iter());
        Ok(())
    }
}

impl ComponentRender for DataGraph {
    fn render(&mut self, context: RenderContext) -> eyre::Result<usvg::Node> {
        let handle = self.handle.as_ref().unwrap();
        let data = handle.read_float(false)?;
        self.samples.pop_front();
        self.samples.push_back(data);

        let mut line = PathData::new();
        let mut fill = PathData::new();
        let mut first_point = None;
        let mut last_point = None;

        for (i, sample) in self.samples.iter().enumerate() {
            if sample.is_nan() {
                continue;
            }

            let x = i as f64 * self.width / (self.sample_count - 1) as f64;
            let y = self.height - (sample - self.min) / (self.max - self.min) * self.height;

            if first_point.is_none() {
                line.push_move_to(x, y);
                fill.push_move_to(x, self.height);
                fill.push_line_to(x, y);
                first_point = Some((x, y));
            } else {
                line.push_line_to(x, y);
                fill.push_line_to(x, y);
            }

            last_point = Some((x, y));
        }

        if let Some((x, y)) = last_point {
            fill.push_line_to(self.width, self.height);
        }
        if let Some((x, y)) = first_point {
            fill.push_line_to(x, self.height);
        }

        let mut group = Node::new(NodeKind::Group(Group::default()));

        group.append(Node::new(NodeKind::Path(Path {
            data: Rc::new(PathData::from_rect(
                Rect::new(0.0, 0.0, self.width, self.height).unwrap_or_log(),
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
