use std::error::Error;

use declarrred::rt::{Data, DataFormat};

#[cfg(target_os = "windows")]
pub mod windows;

pub trait Knowhw {
    type Error: Error;

    fn update(&self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn query(&mut self, query: &str, preferred_format: &DataFormat) -> Result<Data, Self::Error>;
}
