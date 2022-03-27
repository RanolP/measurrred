use std::error::Error;

pub trait Knowhw<Query, Answer> {
    type Error: Error;

    fn update(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn query(&mut self, query: Query) -> Result<Answer, Self::Error>;
}
