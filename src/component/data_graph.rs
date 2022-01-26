use std::{collections::LinkedList, rc::Rc, str::FromStr};

use serde::Deserialize;
use tracing_unwrap::{OptionExt, ResultExt};
use usvg::{Group, Node, NodeKind, Paint, Path, PathData, Rect, Stroke, StrokeWidth};

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

        let mut path_data = PathData::new();
        let mut is_first_data = true;

        for (i, sample) in self.samples.iter().enumerate() {
            if sample.is_nan() {
                continue;
            }

            let x = i as f64 * self.width / self.sample_count as f64;
            let y = self.height - (sample - self.min) / (self.max - self.min) * self.height;

            if is_first_data {
                path_data.push_move_to(x, y);
                is_first_data = false;
            } else {
                path_data.push_line_to(x, y);
            }
        }

        let mut group = Node::new(NodeKind::Group(Group::default()));

        group.append(Node::new(NodeKind::Path(Path {
            data: Rc::new(PathData::from_rect(
                Rect::new(0.0, 0.0, self.width, self.height).unwrap_or_log(),
            )),
            ..Default::default()
        })));

        group.append(Node::new(NodeKind::Path(Path {
            data: Rc::new(path_data),
            stroke: Some(Stroke {
                paint: Paint::Color(Color::from_str("red").unwrap_or_log().to_usvg_color()),
                width: StrokeWidth::new(4.0),
                ..Default::default()
            }),
            ..Default::default()
        })));

        Ok(group)
    }
}
