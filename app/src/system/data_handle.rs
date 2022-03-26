use super::Data;

pub struct DataHandle(pub Box<dyn (Fn() -> eyre::Result<Data>) + Sync + Send>);

impl DataHandle {
    pub fn read(&self) -> eyre::Result<Data> {
        (self.0)()
    }
}
