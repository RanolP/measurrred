use resvg::render_node;
use tiny_skia::{Pixmap, PixmapMut, Transform};
use usvg::{Align, AspectRatio, FitTo, Rect, Size, Svg, Tree};

use crate::{
    component::Component,
    system::{HorizontalPosition, VerticalPosition},
};

pub struct Widget {
    pub x: HorizontalPosition,
    pub y: VerticalPosition,
    pub components: Vec<Component>,
}

impl Widget {
    pub fn render(&mut self, target: &mut Pixmap) -> eyre::Result<()> {
        let tree = Tree::create(Svg {
            size: Size::new(target.width() as f64, target.height() as f64).unwrap(),
            view_box: usvg::ViewBox {
                rect: Rect::new(0.0, 0.0, target.width() as f64, target.height() as f64).unwrap(),
                aspect: AspectRatio {
                    defer: false,
                    align: Align::None,
                    slice: false,
                },
            },
        });
        for component in self.components.iter_mut() {
            let node = component.render();
            render_node(
                &tree,
                &node,
                FitTo::Original,
                Transform::default(),
                target.as_mut(),
            )
            .unwrap();
        }
        Ok(())
    }
}
