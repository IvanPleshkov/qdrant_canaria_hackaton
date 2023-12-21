use std::sync::{Arc, Mutex};

use crate::{
    action::Action,
    entity::{get_all_entity_refs, Entity, EntityRef},
    scene::Scene,
    Error,
};

use super::move_to_item::MoveToItem;

pub struct TakeItem {
    name: String,
    entity_ref: EntityRef,
}

impl Action for TakeItem {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn execute(&self, scene: Arc<Mutex<Scene>>) -> Result<(), Error> {
        let andrey_position = scene.lock().unwrap().get_position(Entity::Andrey);
        let target = self
            .entity_ref
            .objects
            .iter()
            .filter(|entity| entity.is_takeable())
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
            log::error!("Andrey cannot take it");
            return Ok(());
        }

        if scene.lock().unwrap().grabbed_item.is_some() {
            log::error!("Andrey already has something");
            return Ok(());
        }

        let move_command = MoveToItem {
            name: format!("Move to {}", self.entity_ref.name),
            entity_ref: EntityRef {
                name: self.entity_ref.name.clone(),
                objects: vec![target.0],
            },
        };
        move_command.execute(scene.clone())?;

        if target.0 != Entity::Andrey {
            log::info!("ACTION: Andrey takes {}", target.0.get_name());
        }

        scene.lock().unwrap().grabbed_item = Some(target.0);
        scene.lock().unwrap().dropped_items.remove(&target.0);

        Ok(())
    }
}

impl TakeItem {
    pub fn all() -> Vec<Box<dyn Action>> {
        let mut result: Vec<Box<dyn Action>> = vec![];
        for entity_ref in get_all_entity_refs() {
            result.push(Box::new(TakeItem {
                name: format!("Take {}", entity_ref.name),
                entity_ref: entity_ref.clone(),
            }));
            result.push(Box::new(TakeItem {
                name: format!("Grab {}", entity_ref.name),
                entity_ref: entity_ref.clone(),
            }));
            result.push(Box::new(TakeItem {
                name: format!("Steal {}", entity_ref.name),
                entity_ref: entity_ref.clone(),
            }));
        }
        result
    }
}
