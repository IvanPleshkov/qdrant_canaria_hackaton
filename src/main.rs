pub mod action;
pub mod actions;
pub mod embeddings_generator;
pub mod promt_processor;
pub mod qdrant;
pub mod scene;
pub mod scene_object;

use std::sync::{
    mpsc::{self, Sender},
    Arc, Mutex,
};

use eframe::egui;
use promt_processor::start_promt_processor;

pub type Error = String;

fn main() -> Result<(), eframe::Error> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    log::info!("this is a debug {}", "message");
    log::error!("this is printed by default");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "QDrant hackaton",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::<GameApp>::default()
        }),
    )
}

struct GameApp {
    user_promt: Arc<Mutex<String>>,
    promt_processor_trigger: Sender<()>,
}

impl Default for GameApp {
    fn default() -> Self {
        log::info!("Starting app...");
        let user_promt = Arc::new(Mutex::new("".to_owned()));
        let (sender, receiver) = mpsc::channel();
        let _ = start_promt_processor(user_promt.clone(), receiver);
        Self {
            user_promt,
            promt_processor_trigger: sender,
        }
    }
}

impl eframe::App for GameApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.heading("QDrant hackaton");
            ui.horizontal(|ui| {
                let mut user_promt = self.user_promt.try_lock();
                let mut processing_text = "Processing...".to_owned();
                let (enabled, edit_text): (bool, &mut String) = if let Ok(promt) = &mut user_promt {
                    (true, promt)
                } else {
                    (false, &mut processing_text)
                };

                let is_promt_entered = ui.add_enabled_ui(enabled, |ui| {
                    let name_label = ui.label("Promt: ");
                    let text_edit_response = ui
                        .text_edit_singleline(edit_text)
                        .labelled_by(name_label.id);
                    text_edit_response.lost_focus()
                        && text_edit_response
                            .ctx
                            .input(|i| i.key_pressed(egui::Key::Enter))
                });

                if is_promt_entered.inner {
                    // send signal to promt processor
                    self.promt_processor_trigger.send(()).unwrap();
                }
            });
            ui.label("");
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.image(egui::include_image!("assets/ferris.png"));
        });
    }
}
