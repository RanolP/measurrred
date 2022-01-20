use std::fmt;

use serde::{Deserialize, Serialize};

pub enum Length {
    Pixel(i64),
    ViewboxHeight(f64),
    ViewboxWidth(f64),
}

impl Length {
    pub fn from_str<Err: serde::de::Error>(s: impl AsRef<str>) -> Result<Self, Err> {
        let s = s.as_ref();
        if s.ends_with("px") {
            let s = &s[..s.len() - 2];
            s.parse().map(|i| Length::Pixel(i)).map_err(|_| {
                serde::de::Error::custom(format!("Cannot parse {} as i64 while parsing px unit", s))
            })
        } else if s.ends_with("vh") {
            let s = &s[..s.len() - 2];
            s.parse().map(|i| Length::ViewboxHeight(i)).map_err(|_| {
                serde::de::Error::custom(format!("Cannot parse {} as f64 while parsing vh unit", s))
            })
        } else if s.ends_with("vw") {
            let s = &s[..s.len() - 2];
            s.parse().map(|i| Length::ViewboxWidth(i)).map_err(|_| {
                serde::de::Error::custom(format!("Cannot parse {} as f64 while parsing vw unit", s))
            })
        } else {
            Err(serde::de::Error::custom(format!(
                "{} does not match with length syntax",
                s
            )))
        }
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
        Length::from_str(s)
    }
}

impl Length {
    pub fn translate_to_px(&self, viewbox_width: f64, viewbox_height: f64) -> f64 {
        match self {
            Length::Pixel(px) => *px as f64,
            Length::ViewboxHeight(vh) => (vh * viewbox_height / 100.0),
            Length::ViewboxWidth(vw) => (vw * viewbox_width / 100.0),
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
