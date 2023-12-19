use crate::{
    action::Action,
    scene_object::{get_all_entity_refs, EntityRef},
    Error,
};

pub struct MoveToItem {
    entity_ref: EntityRef,
}

impl Action for MoveToItem {
    fn name(&self) -> String {
        format!("Move to {}", self.entity_ref.name)
    }

    fn execute(&self) -> Result<(), Error> {
        Ok(())
    }
}

impl MoveToItem {
    pub fn all() -> Vec<Box<dyn Action>> {
        get_all_entity_refs()
            .into_iter()
            .map(|entity_ref| Box::new(MoveToItem { entity_ref }) as Box<dyn Action>)
            .collect()
    }
}
