use std::collections::HashMap;

use crate::entity::Entity;

const LEFT_RIGHT_SIZE: f32 = -10.0;

pub const HEAD_AMPLITUDE: f32 = 10.0;

pub struct Scene {
    time_instant: std::time::Instant,
    positions: HashMap<Entity, f32>,
    grabbed_item: Option<Entity>,
}

impl Scene {
    pub fn new() -> Self {
        let mut positions = HashMap::new();
        positions.insert(Entity::Andrey, 500.0);
        positions.insert(Entity::Arnaud, 100.0);
        positions.insert(Entity::Kumar, 150.0);
        positions.insert(Entity::Luis, 300.0);
        positions.insert(Entity::Ivan, 900.0);
        positions.insert(Entity::Tim, 750.0);
        positions.insert(Entity::Roman, 650.0);

        positions.insert(Entity::Computer1, -100.0);
        positions.insert(Entity::Computer2, -100.0);
        positions.insert(Entity::Computer3, -100.0);

        positions.insert(Entity::Lamp1, -100.0);
        positions.insert(Entity::Lamp2, -100.0);
        positions.insert(Entity::Lamp3, -100.0);

        positions.insert(Entity::Chair1, -100.0);
        positions.insert(Entity::Chair2, -100.0);
        positions.insert(Entity::Chair3, -100.0);
        positions.insert(Entity::Chair4, -100.0);

        positions.insert(Entity::Plant1, -100.0);
        positions.insert(Entity::Plant2, -100.0);

        positions.insert(Entity::Table1, -100.0);
        positions.insert(Entity::Table2, -100.0);

        positions.insert(Entity::Window1, -100.0);
        positions.insert(Entity::Window2, -100.0);
        positions.insert(Entity::Door, -100.0);

        positions.insert(Entity::Cooler, -100.0);
        positions.insert(Entity::Printer, -100.0);
        positions.insert(Entity::Picture1, -100.0);
        positions.insert(Entity::Picture2, -100.0);

        positions.insert(Entity::Cup1, -100.0);
        positions.insert(Entity::Cup2, -100.0);
        positions.insert(Entity::Cup3, -100.0);
        positions.insert(Entity::Cup4, -100.0);

        positions.insert(Entity::Bottle1, -100.0);
        positions.insert(Entity::Bottle2, -100.0);

        positions.insert(Entity::LeftEnd, -100.0);
        positions.insert(Entity::RightEnd, -100.0);
        positions.insert(Entity::Left, -100.0);
        positions.insert(Entity::Right, -100.0);

        Self {
            time_instant: std::time::Instant::now(),
            positions,
            grabbed_item: Some(Entity::Cup1),
        }
    }

    pub fn update(&mut self) {
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

        egui::Image::new(egui::include_image!("assets/ferris.png"))
            .rounding(5.0)
            .tint(egui::Color32::LIGHT_BLUE)
            .paint_at(ui, 1.5 * full_rect);

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
            egui::Pos2::new(self.get_position(Entity::Ivan), y_line + 20.0),
        );
        self.draw_kumar(
            ui,
            egui::Pos2::new(self.get_position(Entity::Kumar), y_line + 20.0),
        );

        self.draw_andrey(
            ui,
            egui::Pos2::new(self.get_position(Entity::Andrey), y_line + 50.0),
        );
    }

    pub fn set_position(&mut self, entity: Entity, position: f32) {
        self.positions.insert(entity, position);
        if entity == Entity::Andrey {
            let left_position = if position - LEFT_RIGHT_SIZE < self.get_position(Entity::LeftEnd) {
                position - LEFT_RIGHT_SIZE
            } else {
                self.get_position(Entity::LeftEnd)
            };
            self.positions.insert(Entity::Left, left_position);

            let right_position = if position - LEFT_RIGHT_SIZE > self.get_position(Entity::RightEnd)
            {
                position - LEFT_RIGHT_SIZE
            } else {
                self.get_position(Entity::RightEnd)
            };
            self.positions.insert(Entity::Right, right_position);
        }
    }

    pub fn get_position(&self, entity: Entity) -> f32 {
        self.positions.get(&entity).copied().unwrap()
    }

    pub fn draw_andrey(&self, ui: &mut egui::Ui, pos: egui::Pos2) {
        let duration = self.time_instant.elapsed().as_millis() / 500;
        let head_amplitude = duration % 2 == 0;
        let andrey_rect = egui::Rect::from_center_size(
            pos - egui::Vec2::new(0.0, 90.0),
            egui::Vec2::new(200.0, 200.0),
        );
        egui::Image::new(egui::include_image!("assets/andrey_body.png")).paint_at(ui, andrey_rect);
        let andrey_head_rect = egui::Rect::from_center_size(
            pos + egui::Vec2::new(
                -25.0,
                if head_amplitude {
                    -110.0
                } else {
                    -110.0 + HEAD_AMPLITUDE
                },
            ),
            egui::Vec2::new(100.0, 100.0),
        );
        egui::Image::new(egui::include_image!("assets/andrey.png")).paint_at(ui, andrey_head_rect);
    }

    pub fn draw_arnaud(&self, ui: &mut egui::Ui, pos: egui::Pos2) {
        let duration = self.time_instant.elapsed().as_millis() / 500;
        let head_amplitude = duration % 2 == 0;
        let body_rect = egui::Rect::from_center_size(
            pos - egui::Vec2::new(-15.0, 90.0),
            egui::Vec2::new(200.0, 200.0),
        );
        egui::Image::new(egui::include_image!("assets/body.png")).paint_at(ui, body_rect);
        let head_rect = egui::Rect::from_center_size(
            pos + egui::Vec2::new(
                -10.0,
                if head_amplitude {
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
        let duration = self.time_instant.elapsed().as_millis() / 500;
        let head_amplitude = duration % 2 == 0;
        let body_rect = egui::Rect::from_center_size(
            pos - egui::Vec2::new(-15.0, 90.0),
            egui::Vec2::new(200.0, 200.0),
        );
        egui::Image::new(egui::include_image!("assets/body.png")).paint_at(ui, body_rect);
        let head_rect = egui::Rect::from_center_size(
            pos + egui::Vec2::new(
                -10.0,
                if head_amplitude {
                    -120.0
                } else {
                    -120.0 + HEAD_AMPLITUDE
                },
            ),
            egui::Vec2::new(100.0, 100.0),
        );
        egui::Image::new(egui::include_image!("assets/ivan.png")).paint_at(ui, head_rect);
    }

    pub fn draw_luis(&self, ui: &mut egui::Ui, pos: egui::Pos2) {
        let duration = self.time_instant.elapsed().as_millis() / 500;
        let head_amplitude = duration % 2 == 0;
        let body_rect = egui::Rect::from_center_size(
            pos - egui::Vec2::new(-15.0, 90.0),
            egui::Vec2::new(200.0, 200.0),
        );
        egui::Image::new(egui::include_image!("assets/body.png")).paint_at(ui, body_rect);
        let head_rect = egui::Rect::from_center_size(
            pos + egui::Vec2::new(
                -10.0,
                if head_amplitude {
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
        let duration = self.time_instant.elapsed().as_millis() / 500;
        let head_amplitude = duration % 2 == 0;
        let body_rect = egui::Rect::from_center_size(
            pos - egui::Vec2::new(-15.0, 90.0),
            egui::Vec2::new(200.0, 200.0),
        );
        egui::Image::new(egui::include_image!("assets/body.png")).paint_at(ui, body_rect);
        let head_rect = egui::Rect::from_center_size(
            pos + egui::Vec2::new(
                0.0,
                if head_amplitude {
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
        let duration = self.time_instant.elapsed().as_millis() / 500;
        let head_amplitude = duration % 2 == 0;
        let body_rect = egui::Rect::from_center_size(
            pos - egui::Vec2::new(-15.0, 90.0),
            egui::Vec2::new(200.0, 200.0),
        );
        egui::Image::new(egui::include_image!("assets/body.png")).paint_at(ui, body_rect);
        let head_rect = egui::Rect::from_center_size(
            pos + egui::Vec2::new(
                0.0,
                if head_amplitude {
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
        let duration = self.time_instant.elapsed().as_millis() / 500;
        let head_amplitude = duration % 2 == 0;
        let body_rect = egui::Rect::from_center_size(
            pos - egui::Vec2::new(-15.0, 90.0),
            egui::Vec2::new(200.0, 200.0),
        );
        egui::Image::new(egui::include_image!("assets/body.png")).paint_at(ui, body_rect);
        let head_rect = egui::Rect::from_center_size(
            pos + egui::Vec2::new(
                0.0,
                if head_amplitude {
                    -120.0
                } else {
                    -120.0 + HEAD_AMPLITUDE
                },
            ),
            egui::Vec2::new(100.0, 100.0),
        );
        egui::Image::new(egui::include_image!("assets/kumar.png")).paint_at(ui, head_rect);
    }
}
