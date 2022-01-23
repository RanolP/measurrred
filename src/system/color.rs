use std::{fmt, str::FromStr};

use serde::{Deserialize, Serialize};
use svgtypes::Color as SvgColor;
use tiny_skia::Color as TinySkiaColor;
use usvg::Color as UsvgColor;

pub struct Color {
    handle: UsvgColor,
    origin: String,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.origin)
    }
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(&self.to_string())
    }
}

impl FromStr for Color {
    type Err = <SvgColor as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let color = SvgColor::from_str(s)?;

        Ok(Color {
            handle: UsvgColor::new_rgb(color.red, color.green, color.blue),
            origin: s.to_string(),
        })
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Color::from_str(&s)
            .map_err(|_| serde::de::Error::custom(format!("Failed to parse color {}", s)))
    }
}

impl Color {
    pub fn to_usvg_color(&self) -> UsvgColor {
        self.handle.clone()
    }
    pub fn to_tiny_skia_color(&self) -> TinySkiaColor {
        TinySkiaColor::from_rgba8(self.handle.red, self.handle.green, self.handle.blue, 0xFF)
    }
    pub fn to_windows_color(&self) -> u32 {
        (self.handle.red as u32)
            | ((self.handle.green as u32) << 8)
            | ((self.handle.blue as u32) << 16)
    }
}
