use thiserror::Error;
use tiny_skia::{Pixmap, PixmapPaint, Transform};

use crate::{Draw, Renderable, Update};

pub struct RenderEngine<R>
where
    R: Renderable,
{
    width: u32,
    height: u32,
    renderable: R,
    draw_cache: Vec<Draw>,
    piece_indexes: Vec<usize>,
}

impl<R> RenderEngine<R>
where
    R: Renderable,
{
    pub fn combined_scene(&self) -> Pixmap {
        let mut root = Pixmap::new(self.width, self.height).unwrap();

        for index in &self.piece_indexes {
            let Draw { x, y, pixmap, .. } = &self.draw_cache[*index];
            root.draw_pixmap(
                *x,
                *y,
                pixmap.as_ref(),
                &PixmapPaint::default(),
                Transform::default(),
                None,
            );
        }

        root
    }

    pub fn redraw(&mut self) -> Result<(), RenderEngineError<R>> {
        for Update { index, draw } in self
            .renderable
            .render()
            .map_err(|e| RenderEngineError::Render(e))?
        {
            if index == self.draw_cache.len() {
                self.draw_cache.push(draw);
            } else if index < self.draw_cache.len() {
                self.draw_cache[index] = draw;
            } else {
                return Err(RenderEngineError::UpdateIndexOutOfBounds(index));
            }
        }

        let mut new_piece_indexes: Vec<(usize, u32)> = self
            .draw_cache
            .iter()
            .enumerate()
            .map(|(idx, draw)| (idx, draw.z))
            .collect();

        new_piece_indexes.sort_by_cached_key(|(_, z)| *z);

        self.piece_indexes = new_piece_indexes.into_iter().map(|(idx, _)| idx).collect();

        Ok(())
    }
}

#[derive(Error)]
pub enum RenderEngineError<R>
where
    R: Renderable + 'static,
{
    #[error("Failed to render: {0}")]
    Render(#[source] <R as Renderable>::RenderError),
    #[error("Update index out of bounds: {0}")]
    UpdateIndexOutOfBounds(usize),
}
