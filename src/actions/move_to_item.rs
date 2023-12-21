use std::sync::{Arc, Mutex};

use crate::{
    action::Action,
    entity::{get_all_entity_refs, Entity, EntityRef},
    scene::Scene,
    Error,
};

use super::move_to_position;

pub struct MoveToItem {
    pub name: String,
    pub entity_ref: EntityRef,
}

impl Action for MoveToItem {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn execute(&self, scene: Arc<Mutex<Scene>>) -> Result<(), Error> {
        let andrey_position = scene.lock().unwrap().get_position(Entity::Andrey);
        let target =
            self.entity_ref
                .objects
                .iter()
                .fold((Entity::Andrey, f32::MAX), |nearest, &entity| {
                    let position = scene.lock().unwrap().get_position(entity);
                    let distance = (andrey_position - position).abs();
                    if distance < nearest.1 {
                        (entity, distance)
                    } else {
                        nearest
                    }
                });

        if target.1 == f32::MAX {
            log::error!("Andrey cannot go to it");
            return Ok(());
        }

        if target.0 != Entity::Andrey {
            log::info!("ACTION: Andrey goes to {}", target.0.get_name());
        }

        let target_position = scene.lock().unwrap().get_position(target.0);
        move_to_position(scene.clone(), target_position);

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
            result.push(Box::new(MoveToItem {
                name: format!("Stay {}", entity_ref.name),
                entity_ref: entity_ref.clone(),
            }));
        }
        result
    }
}
