use serde::Deserialize;

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
