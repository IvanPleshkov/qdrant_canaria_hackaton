use std::sync::{Arc, Mutex};

use crate::{
    actions::{move_to_item::MoveToItem, take_item::TakeItem, drop_item::DropItem},
    scene::Scene,
    Error,
};

pub trait Action: Send + Sync {
    fn name(&self) -> String;

    fn execute(&self, scene: Arc<Mutex<Scene>>) -> Result<(), Error>;
}

pub struct ActionsCollection {
    pub actions: Vec<Box<dyn Action>>,
}

impl ActionsCollection {
    pub fn new() -> Self {
        let mut actions = vec![];
        actions.extend(MoveToItem::all());
        actions.extend(TakeItem::all());
        actions.extend(DropItem::all());
        Self { actions }
    }
}
