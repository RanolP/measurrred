use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MeasurrredConfig {
    pub foreground_color: String,
    pub background_color: String,
    pub font_family: String,
}
