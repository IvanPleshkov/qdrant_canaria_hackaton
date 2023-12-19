use crate::Error;

pub trait Action: Send + Sync {
    fn name(&self) -> String;

    fn execute(&self) -> Result<(), Error>;
}

pub struct ActionsCollection {
    pub actions: Vec<Box<dyn Action>>,
}

impl ActionsCollection {
    pub fn new() -> Self {
        Self { actions: vec![] }
    }
}
