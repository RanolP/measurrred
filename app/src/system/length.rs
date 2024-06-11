use std::{fmt, str::FromStr};

use serde::{Deserialize, Serialize};
use thiserror::Error;

pub enum Length {
    Pixel(i64),
    ViewboxHeight(f64),
    ViewboxWidth(f64),
}

#[derive(Debug, Error)]
pub enum LengthParseError {
    #[error("Cannot parse {src} as {as_type} while parsing {unit} unit")]
    Parse {
        src: String,
        as_type: &'static str,
        unit: &'static str,
    },
    #[error("{src} does not match with any length syntax")]
    Syntax { src: String },
}

impl FromStr for Length {
    type Err = LengthParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.ends_with("px") {
            let s = &s[..s.len() - 2];
            s.parse()
                .map(|i| Length::Pixel(i))
                .map_err(|_| LengthParseError::Parse {
                    src: s.to_string(),
                    as_type: "i64",
                    unit: "px",
                })
        } else if s.ends_with("vh") {
            let s = &s[..s.len() - 2];
            s.parse()
                .map(|i| Length::ViewboxHeight(i))
                .map_err(|_| LengthParseError::Parse {
                    src: s.to_string(),
                    as_type: "f64",
                    unit: "vh",
                })
        } else if s.ends_with("vw") {
            let s = &s[..s.len() - 2];
            s.parse()
                .map(|i| Length::ViewboxWidth(i))
                .map_err(|_| LengthParseError::Parse {
                    src: s.to_string(),
                    as_type: "f64",
                    unit: "vw",
                })
        } else {
            Err(LengthParseError::Syntax { src: s.to_string() })
        }
    }
}

impl Length {
    pub fn from_str_serde<E: serde::de::Error>(s: impl AsRef<str>) -> Result<Self, E> {
        Length::from_str(s.as_ref()).map_err(|e| E::custom(e))
    }
}

impl fmt::Display for Length {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Length::Pixel(px) => write!(f, "{}px", px),
            Length::ViewboxHeight(vh) => write!(f, "{}vh", vh),
            Length::ViewboxWidth(vw) => write!(f, "{}vw", vw),
        }
    }
}

impl Serialize for Length {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Length {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Length::from_str_serde(&s)
    }
}

impl Length {
    pub fn translate_to_px(&self, viewbox_width: f64, viewbox_height: f64) -> f64 {
        match self {
            Length::Pixel(px) => *px as f64,
            Length::ViewboxHeight(vh) => vh * viewbox_height / 100.0,
            Length::ViewboxWidth(vw) => vw * viewbox_width / 100.0,
        }
    }

    pub fn is_zero(&self) -> bool {
        match self {
            Length::Pixel(px) => *px == 0,
            Length::ViewboxHeight(vh) => vh.abs() < f64::EPSILON,
            Length::ViewboxWidth(vw) => vw.abs() < f64::EPSILON,
        }
    }
}
