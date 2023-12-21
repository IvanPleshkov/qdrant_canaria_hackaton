use std::sync::{Arc, Mutex};

use crate::{
    action::Action,
    entity::{get_all_entity_refs, Entity, EntityRef},
    scene::Scene,
    Error,
};

use super::move_to_item::MoveToItem;

pub struct DropItem {
    name: String,
    destination: Option<EntityRef>,
}

impl Action for DropItem {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn execute(&self, scene: Arc<Mutex<Scene>>) -> Result<(), Error> {
        let andrey_position = scene.lock().unwrap().get_position(Entity::Andrey);

        let grabbed_item = scene.lock().unwrap().grabbed_item.clone();
        if let Some(grabbed_item) = grabbed_item {
            let dst = if let Some(dst) = &self.destination {
                dst
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
                    })
            } else {
                (Entity::Andrey, andrey_position)
            };

            let move_command = MoveToItem {
                name: format!("Move to {}", dst.0.get_name()),
                entity_ref: EntityRef {
                    name: dst.0.get_name().to_owned(),
                    objects: vec![dst.0],
                },
            };
            move_command.execute(scene.clone())?;

            scene.lock().unwrap().grabbed_item = None;
            scene.lock().unwrap().dropped_items.insert(grabbed_item);
            scene.lock().unwrap().set_position(grabbed_item, andrey_position);
        } else {
            log::error!("Nothing to drop");
            return Ok(());
        }

        Ok(())
    }
}

impl DropItem {
    pub fn all() -> Vec<Box<dyn Action>> {
        let mut result: Vec<Box<dyn Action>> = vec![
            Box::new(DropItem {
                name: format!("Drop"),
                destination: None,
            }),
            Box::new(DropItem {
                name: format!("Drop it"),
                destination: None,
            }),
        ];

        for entity_ref in get_all_entity_refs() {
            result.push(Box::new(DropItem {
                name: format!("Give to {}", entity_ref.name),
                destination: Some(entity_ref.clone()),
            }));
            result.push(Box::new(DropItem {
                name: format!("Gift to {}", entity_ref.name),
                destination: Some(entity_ref.clone()),
            }));
            result.push(Box::new(DropItem {
                name: format!("Drop near {}", entity_ref.name),
                destination: Some(entity_ref.clone()),
            }));
        }

        result
    }
}
