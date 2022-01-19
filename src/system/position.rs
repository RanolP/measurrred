use serde::Deserialize;

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
        component_width: f64,
        _component_height: f64,
    ) -> f64 {
        match self {
            HorizontalPosition::Left(length) => {
                HorizontalAlignment::Left.align(viewbox_width, component_width)
                    + length.translate_to_px(viewbox_width, viewbox_height)
            }
            HorizontalPosition::Center => {
                HorizontalAlignment::Center.align(viewbox_width, component_width) + 0.0
            }
            HorizontalPosition::Right(length) => {
                HorizontalAlignment::Right.align(viewbox_width, component_width)
                    + length.translate_to_px(viewbox_width, viewbox_height)
            }
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
        _component_width: f64,
        component_height: f64,
    ) -> f64 {
        match self {
            VerticalPosition::Top(length) => {
                VerticalAlignment::Top.align(viewbox_height, component_height)
                    + length.translate_to_px(viewbox_width, viewbox_height)
            }
            VerticalPosition::Center => {
                VerticalAlignment::Center.align(viewbox_height, component_height) + 0.0
            }
            VerticalPosition::Bottom(length) => {
                VerticalAlignment::Bottom.align(viewbox_height, component_height)
                    + length.translate_to_px(viewbox_width, viewbox_height)
            }
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum HorizontalAlignment {
    Left,
    Center,
    Right,
}

impl HorizontalAlignment {
    pub fn align(&self, viewbox_width: f64, component_width: f64) -> f64 {
        match self {
            HorizontalAlignment::Left => 0.0,
            HorizontalAlignment::Center => viewbox_width / 2.0 - component_width / 2.0,
            HorizontalAlignment::Right => -component_width,
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum VerticalAlignment {
    Top,
    Center,
    Bottom,
}

impl VerticalAlignment {
    pub fn align(&self, viewbox_height: f64, component_height: f64) -> f64 {
        match self {
            VerticalAlignment::Top => 0.0,
            VerticalAlignment::Center => viewbox_height / 2.0 - component_height / 2.0,
            VerticalAlignment::Bottom => -component_height,
        }
    }
}
