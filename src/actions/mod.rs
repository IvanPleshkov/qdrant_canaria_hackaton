use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use crate::{entity::Entity, scene::Scene};

pub mod drop_item;
pub mod move_direction;
pub mod move_to_item;
pub mod take_item;

fn move_to_position(scene: Arc<Mutex<Scene>>, target_position: f32) {
    let mut current_position = scene.lock().unwrap().get_position(Entity::Andrey);
    while (current_position - target_position).abs() > 30.0 {
        current_position += (target_position - current_position).signum() * 5.0;
        std::thread::sleep(Duration::from_millis(20));
        scene
            .lock()
            .unwrap()
            .set_position(Entity::Andrey, current_position);
    }
}
