use resvg::render_node;
use tiny_skia::{Pixmap, Transform};
use usvg::{Align, AspectRatio, FitTo, NodeExt, Options, Rect, Size, Svg, Tree};

use crate::{
    component::{Component, ComponentRender, ComponentSetup, RenderContext, SetupContext},
    config::MeasurrredConfig,
    system::{HorizontalPosition, VerticalPosition},
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
    pub fn new(config: WidgetConfig, components: Component) -> Widget {
        Widget {
            x: config.position.x,
            y: config.position.y,
            component: components,
        }
    }

    pub fn setup(&mut self, context: &mut SetupContext) -> eyre::Result<()> {
        self.component.setup(context)?;

        Ok(())
    }

    pub fn render(
        &self,
        config: &MeasurrredConfig,
        options: &Options,
        target: &mut Pixmap,
    ) -> eyre::Result<()> {
        let viewbox_width = target.width() as f64;
        let viewbox_height = target.height() as f64;

        let context = RenderContext {
            viewbox_width,
            viewbox_height,
            usvg_options: options,
            config,
        };

        let tree = Tree::create(Svg {
            size: Size::new(viewbox_width, viewbox_height).unwrap(),
            view_box: usvg::ViewBox {
                rect: Rect::new(0.0, 0.0, viewbox_width, viewbox_height).unwrap(),
                aspect: AspectRatio {
                    defer: false,
                    align: Align::None,
                    slice: false,
                },
            },
        });

        let node = self.component.render(context.clone())?;
        let bbox = node.calculate_bbox().unwrap();

        let transform = Transform::from_translate(
            self.x
                .to_real_position(viewbox_width, viewbox_height, bbox.width(), bbox.height())
                as f32,
            self.y
                .to_real_position(viewbox_width, viewbox_height, bbox.width(), bbox.height())
                as f32,
        );

        render_node(
            &tree,
            &node,
            FitTo::Original,
            transform.clone(),
            target.as_mut(),
        )
        .unwrap();

        Ok(())
    }
}
