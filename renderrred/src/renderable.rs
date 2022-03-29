use std::error::Error;

use tiny_skia::Pixmap;

use crate::SkipRedraw;

pub trait Renderable {
    type RenderError: Error;

    fn skip_redraw(&self) -> SkipRedraw;

    fn size(&self) -> usize;

    fn render(&self) -> Result<Vec<Update>, Self::RenderError>;
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
