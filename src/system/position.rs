pub enum Length {
    Pixel(i64),
    ViewboxHeight(f64),
    ViewboxWidth(f64),
}

pub enum HorizontalPosition {
    Left(Length),
    Center,
    Right(Length),
}

pub enum VerticalPosition {
    Top(Length),
    Center,
    Bottom(Length),
}

pub enum HorizontalAlignment {
    Left,
    Center,
    Right,
}

pub enum VerticalAlignment {
    Top,
    Center,
    Bottom,
}
