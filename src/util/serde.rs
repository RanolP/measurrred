use std::{fmt, str::FromStr};

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(try_from = "String")]
pub struct FromStrT<F>(pub F)
where
    F: FromStr,
    <F as FromStr>::Err: fmt::Display;

impl<F> TryFrom<String> for FromStrT<F>
where
    F: FromStr,
    <F as FromStr>::Err: fmt::Display,
{
    type Error = <F as FromStr>::Err;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse().map(FromStrT)
    }
}
