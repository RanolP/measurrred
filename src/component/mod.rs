use crate::position::{HorizontalPosition, Length, VerticalPosition};

pub trait Component {
    fn width(&self) -> Length;
    fn height(&self) -> Length;

    fn x(&self) -> HorizontalPosition;
    fn y(&self) -> VerticalPosition;

    fn render(&self);
}
