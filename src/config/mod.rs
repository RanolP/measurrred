use serde::{Deserialize, Serialize};

use crate::system::Color;

#[derive(Serialize, Deserialize)]
pub struct MeasurrredConfig {
    pub foreground_color: Color,
    pub background_color: Color,
    pub font_family: String,
}
