use serde::{Deserialize, Serialize};

use crate::system::{HorizontalPosition, VerticalPosition};

#[derive(Serialize, Deserialize)]
pub struct WidgetConfig {
    pub position: WidgetPositionConfig,
}

#[derive(Serialize, Deserialize)]
pub struct WidgetPositionConfig {
    pub x: HorizontalPosition,
    pub y: VerticalPosition,
}
