use crate::{
    action::Action,
    entity::{get_all_entity_refs, EntityRef},
    Error, scene::Scene,
};

pub struct MoveToItem {
    name: String,
    entity_ref: EntityRef,
}

impl Action for MoveToItem {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn execute(&self, _scene: &mut Scene) -> Result<(), Error> {
        Ok(())
    }
}

impl MoveToItem {
    pub fn all() -> Vec<Box<dyn Action>> {
        let mut result: Vec<Box<dyn Action>> = vec![];
        for entity_ref in get_all_entity_refs() {
            result.push(Box::new(MoveToItem {
                name: format!("Move to {}", entity_ref.name),
                entity_ref: entity_ref.clone(),
            }));
            result.push(Box::new(MoveToItem {
                name: format!("Run to {}", entity_ref.name),
                entity_ref: entity_ref.clone(),
            }));
            result.push(Box::new(MoveToItem {
                name: format!("Go to {}", entity_ref.name),
                entity_ref: entity_ref.clone(),
            }));
        }
        result
    }
}
