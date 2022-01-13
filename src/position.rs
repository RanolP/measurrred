pub enum Length {
    Pixel(i64),
    Percet(f64),
}

pub enum HorizontalPosition {
    Left(Length),
    Right(Length),
}

pub enum VerticalPosition {
    Top(Length),
    Bottom(Length),
}
