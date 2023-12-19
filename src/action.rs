use crate::{Error, actions::move_to_item::MoveToItem, scene::Scene};

pub trait Action: Send + Sync {
    fn name(&self) -> String;

    fn execute(&self, scene: &mut Scene) -> Result<(), Error>;
}

pub struct ActionsCollection {
    pub actions: Vec<Box<dyn Action>>,
}

impl ActionsCollection {
    pub fn new() -> Self {
        let mut actions = vec![];
        actions.extend(MoveToItem::all());
        Self {
            actions,
        }
    }
}
