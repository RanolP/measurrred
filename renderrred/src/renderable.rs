use std::error::Error;

use tiny_skia::Pixmap;

pub trait Renderable {
    type RenderError: Error;

    fn skip_redraw(&self) -> SkipRedraw;

    fn size(&self) -> usize;

    fn render(&self) -> Result<Vec<Update>, Self::RenderError>;
}

pub enum SkipRedraw {
    Never,
    VariablesUnchanged(Vec<String>),
    Always,
}

impl SkipRedraw {
    pub fn combine_with(self, other: SkipRedraw) -> SkipRedraw {
        match (self, other) {
            (SkipRedraw::Never, _) | (_, SkipRedraw::Never) => SkipRedraw::Never,
            (SkipRedraw::VariablesUnchanged(mut a), SkipRedraw::VariablesUnchanged(mut b)) => {
                a.append(&mut b);
                SkipRedraw::VariablesUnchanged(a)
            }
            (SkipRedraw::Always, other) | (other, SkipRedraw::Always) => other,
        }
    }
}

#[derive(Clone)]
pub struct Draw {
    pub x: i32,
    pub y: i32,
    pub z: u32,
    pub pixmap: Pixmap,
}

pub struct Update {
    pub index: usize,
    pub draw: Draw,
}
