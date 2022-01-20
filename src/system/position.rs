use core::fmt;

use serde::{Deserialize, Serialize};

use super::{HorizontalAlignment, Length, VerticalAlignment};

pub enum HorizontalPosition {
    Left(Length),
    Center,
    Right(Length),
}

impl fmt::Display for HorizontalPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HorizontalPosition::Left(len) => {
                if len.is_zero() {
                    write!(f, "left")
                } else {
                    write!(f, "left {}", len)
                }
            }
            HorizontalPosition::Center => write!(f, "center"),
            HorizontalPosition::Right(len) => {
                if len.is_zero() {
                    write!(f, "right")
                } else {
                    write!(f, "right {}", len)
                }
            }
        }
    }
}

impl Serialize for HorizontalPosition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for HorizontalPosition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let err = Err(serde::de::Error::custom(format!(
            "{} does not match with horizontal position syntax",
            s
        )));
        if s.is_empty() {
            return err;
        }
        let splitted: Vec<_> = s.split(" ").collect();
        match splitted[0] {
            "left" => match splitted.len() {
                1 => Ok(HorizontalPosition::Left(Length::Pixel(0))),
                2 => Length::from_str(splitted[1]).map(|len| HorizontalPosition::Left(len)),
                _ => err,
            },
            "center" => match splitted.len() {
                1 => Ok(HorizontalPosition::Center),
                _ => err,
            },
            "right" => match splitted.len() {
                1 => Ok(HorizontalPosition::Right(Length::Pixel(0))),
                2 => Length::from_str(splitted[1]).map(|len| HorizontalPosition::Right(len)),
                _ => err,
            },
            _ => err,
        }
    }
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

impl fmt::Display for VerticalPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VerticalPosition::Top(len) => {
                if len.is_zero() {
                    write!(f, "top")
                } else {
                    write!(f, "top {}", len)
                }
            }
            VerticalPosition::Center => write!(f, "center"),
            VerticalPosition::Bottom(len) => {
                if len.is_zero() {
                    write!(f, "bottom")
                } else {
                    write!(f, "bottom {}", len)
                }
            }
        }
    }
}

impl Serialize for VerticalPosition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for VerticalPosition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let err = Err(serde::de::Error::custom(format!(
            "{} does not match with vertical position syntax",
            s
        )));
        if s.is_empty() {
            return err;
        }
        let splitted: Vec<_> = s.split(" ").collect();
        match splitted[0] {
            "top" => match splitted.len() {
                1 => Ok(VerticalPosition::Top(Length::Pixel(0))),
                2 => Length::from_str(splitted[1]).map(|len| VerticalPosition::Top(len)),
                _ => err,
            },
            "center" => match splitted.len() {
                1 => Ok(VerticalPosition::Center),
                _ => err,
            },
            "bottom" => match splitted.len() {
                1 => Ok(VerticalPosition::Bottom(Length::Pixel(0))),
                2 => Length::from_str(splitted[1]).map(|len| VerticalPosition::Bottom(len)),
                _ => err,
            },
            _ => err,
        }
    }
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
