use resvg::render_node;
use tiny_skia::{Pixmap, Transform};
use usvg::{Align, AspectRatio, FitTo, NodeExt, Options, Rect, Size, Svg, Tree};

use crate::{
    component::{Component, ComponentRender, ComponentSetup, RenderContext, SetupContext},
    config::MeasurrredConfig,
    system::{HorizontalPosition, VerticalPosition},
};

pub struct Widget {
    pub x: HorizontalPosition,
    pub y: VerticalPosition,
    pub components: Vec<Component>,
}

impl Widget {
    pub fn setup(&mut self, context: &mut SetupContext) -> eyre::Result<()> {
        for component in self.components.iter_mut() {
            component.setup(context)?;
        }

        Ok(())
    }

    pub fn render(&self, options: &Options, target: &mut Pixmap) -> eyre::Result<()> {
        let viewbox_width = target.width() as f64;
        let viewbox_height = target.height() as f64;

        let config = MeasurrredConfig {
            foreground_color: "white".to_string(),
            background_color: "black".to_string(),
            font_family: "Noto Sans CJK KR Bold".to_string(),
        };
        let config = &config;

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
        let mut nodes = Vec::new();
        let mut mostleft = 0.0;
        let mut mostright = 0.0;
        let mut mosttop = 0.0;
        let mut mostbottom = 0.0;
        for component in &self.components {
            let node = component.render(context.clone())?;
            let bbox = node.calculate_bbox().unwrap();
            mostleft = f64::min(mostleft, bbox.left());
            mostright = f64::max(mostright, bbox.right());
            mosttop = f64::min(mosttop, bbox.top());
            mostbottom = f64::max(mostbottom, bbox.bottom());
            nodes.push(node);
        }
        let total_width = mostright - mostleft;
        let total_height = mostbottom - mosttop;

        let transform = Transform::from_translate(
            self.x
                .to_real_position(viewbox_width, viewbox_height, total_width, total_height)
                as f32,
            self.y
                .to_real_position(viewbox_width, viewbox_height, total_width, total_height)
                as f32,
        );
        for node in nodes {
            render_node(
                &tree,
                &node,
                FitTo::Original,
                transform.clone(),
                target.as_mut(),
            )
            .unwrap();
        }
        Ok(())
    }
}
