use std::collections::HashMap;

use declarrred::rt::Data;
use futures::{future::join_all, StreamExt};
use tracing::info;
use usvg::NodeExt;

use crate::{
    component::{
        Component, ComponentAction, Job, JobStage, RenderContext, SetupContext, UpdateContext,
    },
    config::MeasurrredConfig,
    system::{HorizontalPosition, Rect, VerticalPosition},
};

pub use self::config::WidgetConfig;
pub use self::loader::*;

mod config;
mod loader;

pub struct Widget {
    pub x: HorizontalPosition,
    pub y: VerticalPosition,
    pub component: Component,
}

impl Widget {
    pub fn new(config: WidgetConfig, component: Component) -> Self {
        Widget {
            x: config.position.x,
            y: config.position.y,
            component,
        }
    }

    pub async fn setup<'a>(&'a mut self, context: &mut SetupContext) -> eyre::Result<()> {
        async fn process(
            (id, mut job): (usize, Job),
        ) -> eyre::Result<(
            usize,
            Option<Box<dyn FnOnce(&mut SetupContext) -> eyre::Result<()> + 'static + Send>>,
        )> {
            while let Some(stage) = job.next().await.transpose()? {
                info!("#{}: {}", id, stage.label());

                match stage {
                    JobStage::Completed { finalizer, .. } => return Ok((id, Some(finalizer))),
                    JobStage::Progress { .. } => {}
                    JobStage::Fail { label } => eyre::bail!("{}", label),
                }
            }
            Ok((id, None))
        }
        let jobs = self.component.setup();
        let finalizers: Vec<_> = jobs
            .into_iter()
            .enumerate()
            .map(process)
            .map(async_std::task::spawn)
            .collect();
        let finalizers = join_all(finalizers).await;
        for finalizer in finalizers {
            if let (id, Some(finalizer)) = finalizer? {
                info!("Finalizing #{}...", id);
                finalizer(context)?;
            }
        }

        Ok(())
    }

    pub fn render(
        &mut self,
        measurred_config: &MeasurrredConfig,
        usvg_options: &usvg::Options,
        target: &mut tiny_skia::Pixmap,
        viewbox: Rect,
        zoom: f32,
        variables: &HashMap<String, Data>,
    ) -> eyre::Result<()> {
        let viewbox_width = viewbox.width() as f64;
        let viewbox_height = viewbox.height() as f64;

        let mut update_context = UpdateContext::new(measurred_config);
        self.component.update(&mut update_context)?;

        let render_context = RenderContext::new(
            viewbox_width / zoom as f64,
            viewbox_height / zoom as f64,
            usvg_options,
            measurred_config,
            variables,
        );
        let root = self.component.render(&render_context)?;

        let tree = usvg::Tree::create(usvg::Svg {
            size: usvg::Size::new(viewbox_width, viewbox_height).unwrap(),
            view_box: usvg::ViewBox {
                rect: usvg::Rect::new(0.0, 0.0, viewbox_width, viewbox_height).unwrap(),
                aspect: usvg::AspectRatio {
                    defer: false,
                    align: usvg::Align::None,
                    slice: false,
                },
            },
        });

        let bbox = root.calculate_bbox().unwrap();
        let actual_width = bbox.width();
        let actual_height = bbox.height();

        let transform = tiny_skia::Transform::from_row(
            zoom,
            0.0,
            0.0,
            zoom,
            self.x.to_real_position(
                viewbox_width,
                viewbox_height,
                actual_width * zoom as f64,
                actual_height * zoom as f64,
            ) as f32,
            self.y.to_real_position(
                viewbox_width,
                viewbox_height,
                actual_width * zoom as f64,
                actual_height * zoom as f64,
            ) as f32,
        );

        resvg::render_node(
            &tree,
            &root,
            usvg::FitTo::Original,
            transform.clone(),
            target.as_mut(),
        )
        .unwrap();

        Ok(())
    }
}
