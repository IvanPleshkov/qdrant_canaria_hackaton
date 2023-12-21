use std::collections::HashMap;

use egui::ahash::HashSet;

use crate::entity::Entity;

pub const HEAD_AMPLITUDE: f32 = 10.0;

pub struct Scene {
    time_instant: std::time::Instant,
    positions: HashMap<Entity, f32>,
    pub dropped_items: HashSet<Entity>,
    pub grabbed_item: Option<Entity>,
    pub ping_pong: bool,
}

impl Scene {
    pub fn new() -> Self {
        let mut positions = HashMap::new();
        positions.insert(Entity::Andrey, 500.0);
        positions.insert(Entity::Arnaud, 100.0);
        positions.insert(Entity::Kumar, 150.0);
        positions.insert(Entity::Luis, 300.0);
        positions.insert(Entity::Ivan, 1000.0);
        positions.insert(Entity::Tim, 850.0);
        positions.insert(Entity::Roman, 750.0);

        positions.insert(Entity::Window1, 600.0);
        positions.insert(Entity::Window2, 800.0);

        positions.insert(Entity::Cup1, 100.0 - 50.0); // cup1 - Arnaud cup
        positions.insert(Entity::Cup2, 300.0 - 50.0); // cup2 - Luis cup
        positions.insert(Entity::Cup3, 750.0 - 50.0); // cup3 - Roman cup
        positions.insert(Entity::Cup4, 850.0 - 50.0); // cup4 - Tim cup

        positions.insert(Entity::Bottle1, 1000.0);
        positions.insert(Entity::Bottle2, 1020.0);

        Self {
            time_instant: std::time::Instant::now(),
            positions,
            grabbed_item: None,
            dropped_items: HashSet::default(),
            ping_pong: true,
        }
    }

    pub fn update(&mut self) {
        let duration = self.time_instant.elapsed().as_millis() / 500;
        self.ping_pong = duration % 2 == 0;

        if let Some(grabbed_item) = self.grabbed_item {
            let andrey_position = self.get_position(Entity::Andrey);
            self.set_position(grabbed_item, andrey_position);
        }
    }

    pub fn render(&self, ui: &mut egui::Ui) {
        let full_rect = egui::Rect::from_min_size(ui.next_widget_position(), ui.available_size());
        let center = full_rect.center();
        let y_line = center.y + 100.0;
        ui.set_clip_rect(full_rect);

        egui::Image::new(egui::include_image!("assets/floor_texture.png")).paint_at(
            ui,
            egui::Rect::from_two_pos(egui::Pos2::new(0.0, y_line), full_rect.right_bottom()),
        );
        egui::Image::new(egui::include_image!("assets/wall_texture.png")).paint_at(
            ui,
            egui::Rect::from_two_pos(
                egui::Pos2::new(0.0, 0.0),
                egui::Pos2::new(full_rect.right_bottom().x, y_line),
            ),
        );
        egui::Image::new(egui::include_image!("assets/window.png")).paint_at(
            ui,
            egui::Rect::from_center_size(
                egui::Pos2::new(self.get_position(Entity::Window1) - 200.0, y_line - 100.0),
                egui::Vec2::new(100.0, 100.0),
            ),
        );
        egui::Image::new(egui::include_image!("assets/window.png")).paint_at(
            ui,
            egui::Rect::from_center_size(
                egui::Pos2::new(self.get_position(Entity::Window2) - 200.0, y_line - 100.0),
                egui::Vec2::new(100.0, 100.0),
            ),
        );

        self.draw_arnaud(
            ui,
            egui::Pos2::new(self.get_position(Entity::Arnaud), y_line + 20.0),
        );
        self.draw_luis(
            ui,
            egui::Pos2::new(self.get_position(Entity::Luis), y_line + 20.0),
        );
        self.draw_tim(
            ui,
            egui::Pos2::new(self.get_position(Entity::Tim), y_line + 20.0),
        );
        self.draw_roman(
            ui,
            egui::Pos2::new(self.get_position(Entity::Roman), y_line + 20.0),
        );
        self.draw_ivan(
            ui,
            egui::Pos2::new(self.get_position(Entity::Ivan), y_line - 50.0),
        );
        self.draw_kumar(
            ui,
            egui::Pos2::new(self.get_position(Entity::Kumar), y_line + 20.0),
        );

        if !self.dropped_items.contains(&Entity::Cup1) && self.grabbed_item != Some(Entity::Cup1) {
            self.draw_cup(
                ui,
                egui::Pos2::new(self.get_position(Entity::Cup1), y_line - 7.0),
            );
        }
        if !self.dropped_items.contains(&Entity::Cup2) && self.grabbed_item != Some(Entity::Cup2) {
            self.draw_cup(
                ui,
                egui::Pos2::new(self.get_position(Entity::Cup2), y_line - 7.0),
            );
        }
        if !self.dropped_items.contains(&Entity::Cup3) && self.grabbed_item != Some(Entity::Cup3) {
            self.draw_cup(
                ui,
                egui::Pos2::new(self.get_position(Entity::Cup3), y_line - 10.0),
            );
        }
        if !self.dropped_items.contains(&Entity::Cup4) && self.grabbed_item != Some(Entity::Cup4) {
            self.draw_cup(
                ui,
                egui::Pos2::new(self.get_position(Entity::Cup4), y_line - 10.0),
            );
        }
        if !self.dropped_items.contains(&Entity::Bottle1)
            && self.grabbed_item != Some(Entity::Bottle1)
        {
            self.draw_bottle(
                ui,
                egui::Pos2::new(self.get_position(Entity::Bottle1), y_line + 20.0),
            );
        }
        if !self.dropped_items.contains(&Entity::Bottle2)
            && self.grabbed_item != Some(Entity::Bottle2)
        {
            self.draw_bottle(
                ui,
                egui::Pos2::new(self.get_position(Entity::Bottle2), y_line + 20.0),
            );
        }

        self.draw_andrey(
            ui,
            egui::Pos2::new(self.get_position(Entity::Andrey), y_line + 50.0),
        );

        for dropped_item in &self.dropped_items {
            let position = self.get_position(*dropped_item);
            let y_line = y_line + 60.0;
            match dropped_item {
                Entity::Cup1 => self.draw_cup(ui, egui::Pos2::new(position, y_line)),
                Entity::Cup2 => self.draw_cup(ui, egui::Pos2::new(position, y_line)),
                Entity::Cup3 => self.draw_cup(ui, egui::Pos2::new(position, y_line)),
                Entity::Cup4 => self.draw_cup(ui, egui::Pos2::new(position, y_line)),
                Entity::Bottle1 => self.draw_bottle(ui, egui::Pos2::new(position, y_line)),
                Entity::Bottle2 => self.draw_bottle(ui, egui::Pos2::new(position, y_line)),
                _ => {}
            }
        }
    }

    pub fn set_position(&mut self, entity: Entity, position: f32) {
        self.positions.insert(entity, position);
    }

    pub fn get_position(&self, entity: Entity) -> f32 {
        self.positions.get(&entity).copied().unwrap()
    }

    pub fn draw_andrey(&self, ui: &mut egui::Ui, pos: egui::Pos2) {
        let andrey_rect = egui::Rect::from_center_size(
            pos - egui::Vec2::new(0.0, 90.0),
            egui::Vec2::new(200.0, 200.0),
        );
        egui::Image::new(egui::include_image!("assets/andrey_body.png")).paint_at(ui, andrey_rect);
        let andrey_head_rect = egui::Rect::from_center_size(
            pos + egui::Vec2::new(
                -25.0,
                if self.ping_pong {
                    -110.0
                } else {
                    -110.0 + HEAD_AMPLITUDE
                },
            ),
            egui::Vec2::new(100.0, 100.0),
        );
        egui::Image::new(egui::include_image!("assets/andrey.png")).paint_at(ui, andrey_head_rect);

        if let Some(grabbed_item) = self.grabbed_item {
            let hand_position = pos - egui::Vec2::new(0.0, 45.0);
            match grabbed_item {
                Entity::Cup1 => self.draw_cup(ui, hand_position),
                Entity::Cup2 => self.draw_cup(ui, hand_position),
                Entity::Cup3 => self.draw_cup(ui, hand_position),
                Entity::Cup4 => self.draw_cup(ui, hand_position),
                Entity::Bottle1 => self.draw_bottle(ui, hand_position),
                Entity::Bottle2 => self.draw_bottle(ui, hand_position),
                _ => {}
            }
        }
    }

    pub fn draw_arnaud(&self, ui: &mut egui::Ui, pos: egui::Pos2) {
        let body_rect = egui::Rect::from_center_size(
            pos - egui::Vec2::new(-15.0, 90.0),
            egui::Vec2::new(200.0, 200.0),
        );
        egui::Image::new(egui::include_image!("assets/body.png")).paint_at(ui, body_rect);
        let head_rect = egui::Rect::from_center_size(
            pos + egui::Vec2::new(
                -10.0,
                if self.ping_pong {
                    -120.0
                } else {
                    -120.0 + HEAD_AMPLITUDE
                },
            ),
            egui::Vec2::new(100.0, 100.0),
        );
        egui::Image::new(egui::include_image!("assets/arnauld.png")).paint_at(ui, head_rect);
    }

    pub fn draw_ivan(&self, ui: &mut egui::Ui, pos: egui::Pos2) {
        let rope_rect = egui::Rect::from_two_pos(
            pos + egui::Vec2::new(0.0, -120.0),
            egui::pos2(pos.x + 5.0, 0.0),
        );
        egui::Image::new(egui::include_image!("assets/floor_texture.png")).paint_at(ui, rope_rect);
        let body_rect = egui::Rect::from_center_size(
            pos - egui::Vec2::new(-15.0, 90.0),
            egui::Vec2::new(200.0, 200.0),
        );
        egui::Image::new(egui::include_image!("assets/ivan_body.png")).paint_at(ui, body_rect);
        let head_rect = egui::Rect::from_center_size(
            pos + egui::Vec2::new(0.0, -120.0),
            egui::Vec2::new(100.0, 100.0),
        );
        egui::Image::new(egui::include_image!("assets/ivan.png")).paint_at(ui, head_rect);
    }

    pub fn draw_luis(&self, ui: &mut egui::Ui, pos: egui::Pos2) {
        let body_rect = egui::Rect::from_center_size(
            pos - egui::Vec2::new(-15.0, 90.0),
            egui::Vec2::new(200.0, 200.0),
        );
        egui::Image::new(egui::include_image!("assets/body.png")).paint_at(ui, body_rect);
        let head_rect = egui::Rect::from_center_size(
            pos + egui::Vec2::new(
                0.0,
                if self.ping_pong {
                    -120.0
                } else {
                    -120.0 + HEAD_AMPLITUDE
                },
            ),
            egui::Vec2::new(100.0, 100.0),
        );
        egui::Image::new(egui::include_image!("assets/luis.png")).paint_at(ui, head_rect);
    }

    pub fn draw_tim(&self, ui: &mut egui::Ui, pos: egui::Pos2) {
        let body_rect = egui::Rect::from_center_size(
            pos - egui::Vec2::new(-15.0, 90.0),
            egui::Vec2::new(200.0, 200.0),
        );
        egui::Image::new(egui::include_image!("assets/body.png")).paint_at(ui, body_rect);
        let head_rect = egui::Rect::from_center_size(
            pos + egui::Vec2::new(
                10.0,
                if self.ping_pong {
                    -120.0
                } else {
                    -120.0 + HEAD_AMPLITUDE
                },
            ),
            egui::Vec2::new(100.0, 100.0),
        );
        egui::Image::new(egui::include_image!("assets/tim.png")).paint_at(ui, head_rect);
    }

    pub fn draw_roman(&self, ui: &mut egui::Ui, pos: egui::Pos2) {
        let body_rect = egui::Rect::from_center_size(
            pos - egui::Vec2::new(-15.0, 90.0),
            egui::Vec2::new(200.0, 200.0),
        );
        egui::Image::new(egui::include_image!("assets/body.png")).paint_at(ui, body_rect);
        let head_rect = egui::Rect::from_center_size(
            pos + egui::Vec2::new(
                0.0,
                if self.ping_pong {
                    -120.0
                } else {
                    -120.0 + HEAD_AMPLITUDE
                },
            ),
            egui::Vec2::new(100.0, 100.0),
        );
        egui::Image::new(egui::include_image!("assets/roman.png")).paint_at(ui, head_rect);
    }

    pub fn draw_kumar(&self, ui: &mut egui::Ui, pos: egui::Pos2) {
        let body_rect = egui::Rect::from_center_size(
            pos - egui::Vec2::new(-15.0, 90.0),
            egui::Vec2::new(200.0, 200.0),
        );
        egui::Image::new(egui::include_image!("assets/body.png")).paint_at(ui, body_rect);
        let head_rect = egui::Rect::from_center_size(
            pos + egui::Vec2::new(
                0.0,
                if self.ping_pong {
                    -120.0
                } else {
                    -120.0 + HEAD_AMPLITUDE
                },
            ),
            egui::Vec2::new(100.0, 100.0),
        );
        egui::Image::new(egui::include_image!("assets/kumar.png")).paint_at(ui, head_rect);
    }

    pub fn draw_cup(&self, ui: &mut egui::Ui, pos: egui::Pos2) {
        let rect = egui::Rect::from_center_size(
            pos - egui::Vec2::new(0.0, 20.0),
            egui::Vec2::new(40.0, 40.0),
        );
        egui::Image::new(egui::include_image!("assets/cup.png")).paint_at(ui, rect);
    }

    pub fn draw_bottle(&self, ui: &mut egui::Ui, pos: egui::Pos2) {
        let rect = egui::Rect::from_center_size(
            pos - egui::Vec2::new(0.0, 30.0),
            egui::Vec2::new(60.0, 60.0),
        );
        egui::Image::new(egui::include_image!("assets/bottle.png")).paint_at(ui, rect);
    }
}
