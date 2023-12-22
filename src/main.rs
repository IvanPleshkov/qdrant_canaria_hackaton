pub mod action;
pub mod actions;
pub mod embeddings_generator;
pub mod entity;
pub mod history_analysis;
pub mod promt_processor;
pub mod qdrant;
pub mod scene;

use std::sync::{
    mpsc::{self, Sender},
    Arc, Mutex,
};

use clap::Parser;
use eframe::egui;
use promt_processor::start_promt_processor;
use scene::Scene;

/// Simple program to greet a person
#[derive(Parser, Debug)]
struct Args {
    #[clap(long, default_value_t = false)]
    pub recreate: bool,

    #[clap(long)]
    pub analysis: Option<String>,
}

pub type Error = String;

fn main() -> Result<(), eframe::Error> {
    let args = Args::parse();

    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    if let Some(filename) = &args.analysis {
        history_analysis::run(&filename);
        return Ok(());
    }

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1100.0, 500.0]),
        ..Default::default()
    };
    eframe::run_native(
        "QDrant hackaton",
        options,
        Box::new(move |cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::new(GameApp::init(args.recreate))
        }),
    )
}

struct GameApp {
    user_promt: Arc<Mutex<String>>,
    scene: Arc<Mutex<Scene>>,
    promt_processor_trigger: Sender<()>,
}

impl GameApp {
    fn init(recreate_queries: bool) -> Self {
        log::info!("Starting app...");
        let user_promt = Arc::new(Mutex::new("".to_owned()));
        let scene = Arc::new(Mutex::new(Scene::new()));
        let (sender, receiver) = mpsc::channel();
        let _ = start_promt_processor(
            recreate_queries,
            user_promt.clone(),
            scene.clone(),
            receiver,
        );
        Self {
            user_promt,
            scene,
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
            self.scene.lock().unwrap().update();
            self.scene.lock().unwrap().render(ui);
        });

        ctx.request_repaint();
    }
}
