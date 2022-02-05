use serde::{Deserialize, Serialize};

use crate::system::{HorizontalPosition, VerticalPosition};

#[derive(Serialize, Deserialize)]
pub struct WidgetConfig {
    pub general: GeneralSection,
    pub position: PositionSection,
}

#[derive(Serialize, Deserialize)]
pub struct GeneralSection {
    #[serde(default = "default_general_enabled")]
    pub enabled: bool,
}

fn default_general_enabled() -> bool {
    true
}

#[derive(Serialize, Deserialize)]
pub struct PositionSection {
    pub x: HorizontalPosition,
    pub y: VerticalPosition,
}
