use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use crate::{
    action::Action,
    entity::{get_all_entity_refs, Entity, EntityRef},
    scene::Scene,
    Error,
};

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

        let mut current_position = scene.lock().unwrap().get_position(Entity::Andrey);
        let target_position = scene.lock().unwrap().get_position(target.0);
        while (current_position - target_position).abs() > 30.0 {
            current_position += (target_position - current_position).signum() * 5.0;
            std::thread::sleep(Duration::from_millis(20));
            scene
                .lock()
                .unwrap()
                .set_position(Entity::Andrey, current_position);
        }
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
