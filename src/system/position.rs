pub enum Length {
    Pixel(i64),
    ViewboxHeight(f64),
    ViewboxWidth(f64),
}

impl Length {
    pub fn translate_to_px(&self, viewbox_width: f64, viewbox_height: f64) -> f64 {
        match self {
            Length::Pixel(px) => px.clone() as f64,
            Length::ViewboxHeight(percent) => (percent * viewbox_height / 100.0),
            Length::ViewboxWidth(percent) => (percent * viewbox_width / 100.0),
        }
    }
}

pub enum HorizontalPosition {
    Left(Length),
    Center,
    Right(Length),
}

impl HorizontalPosition {
    pub fn to_real_position(
        &self,
        viewbox_width: f64,
        viewbox_height: f64,
    ) -> (HorizontalAlignment, f64) {
        match self {
            HorizontalPosition::Left(length) => (
                HorizontalAlignment::Left,
                length.translate_to_px(viewbox_width, viewbox_height),
            ),
            HorizontalPosition::Center => (HorizontalAlignment::Center, 0.0),
            HorizontalPosition::Right(length) => (
                HorizontalAlignment::Right,
                length.translate_to_px(viewbox_width, viewbox_height),
            ),
        }
    }
}

pub enum VerticalPosition {
    Top(Length),
    Center,
    Bottom(Length),
}

impl VerticalPosition {
    pub fn to_real_position(
        &self,
        viewbox_width: f64,
        viewbox_height: f64,
    ) -> (VerticalAlignment, f64) {
        match self {
            VerticalPosition::Top(length) => (
                VerticalAlignment::Top,
                length.translate_to_px(viewbox_width, viewbox_height),
            ),
            VerticalPosition::Center => (VerticalAlignment::Center, 0.0),
            VerticalPosition::Bottom(length) => (
                VerticalAlignment::Bottom,
                length.translate_to_px(viewbox_width, viewbox_height),
            ),
        }
    }
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
