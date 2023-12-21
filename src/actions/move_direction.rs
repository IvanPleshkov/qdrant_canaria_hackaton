use std::sync::{Arc, Mutex};

use crate::{action::Action, entity::Entity, scene::Scene, Error};

use super::move_to_position;

pub enum Direction {
    Left,
    Right,
    LeftCorner,
    RightCorner,
}

pub struct MoveDirection {
    pub name: String,
    pub direction: Direction,
}

impl Action for MoveDirection {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn execute(&self, scene: Arc<Mutex<Scene>>) -> Result<(), Error> {
        let andrey_position = scene.lock().unwrap().get_position(Entity::Andrey);
        let target_position = match self.direction {
            Direction::Left => {
                log::info!("ACTION: Andrey goes left");
                andrey_position - 100.0
            }
            Direction::Right => {
                log::info!("ACTION: Andrey goes right");
                andrey_position + 100.0
            }
            Direction::LeftCorner => {
                log::info!("ACTION: Andrey goes left corner");
                0.0
            }
            Direction::RightCorner => {
                log::info!("ACTION: Andrey goes right corner");
                1000.0
            }
        };
        let target_position = target_position.max(0.0).min(1000.0);
        move_to_position(scene.clone(), target_position);
        Ok(())
    }
}

impl MoveDirection {
    pub fn all() -> Vec<Box<dyn Action>> {
        let mut result: Vec<Box<dyn Action>> = vec![];
        result.push(Box::new(MoveDirection {
            name: format!("Move left"),
            direction: Direction::Left,
        }));
        result.push(Box::new(MoveDirection {
            name: format!("Go left"),
            direction: Direction::Left,
        }));
        result.push(Box::new(MoveDirection {
            name: format!("Move right"),
            direction: Direction::Right,
        }));
        result.push(Box::new(MoveDirection {
            name: format!("Go right"),
            direction: Direction::Right,
        }));
        result.push(Box::new(MoveDirection {
            name: format!("Move left corner"),
            direction: Direction::LeftCorner,
        }));
        result.push(Box::new(MoveDirection {
            name: format!("Go left corner"),
            direction: Direction::LeftCorner,
        }));
        result.push(Box::new(MoveDirection {
            name: format!("Move right corner"),
            direction: Direction::RightCorner,
        }));
        result.push(Box::new(MoveDirection {
            name: format!("Go right corner"),
            direction: Direction::RightCorner,
        }));
        result
    }
}
