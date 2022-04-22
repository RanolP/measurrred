use std::error::Error;

#[cfg(target_os = "windows")]
pub mod windows;

pub trait Knowhw<Query, Answer> {
    type Error: Error;

    fn update(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn query(&mut self, query: &Query) -> Result<Answer, Self::Error>;
}
