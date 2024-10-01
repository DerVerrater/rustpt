#![warn(clippy::all, rust_2018_idioms)]
use eframe::{egui, Frame};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([50.0, 50.0])
            .with_title("RustPT GUI Tool"),
        ..Default::default()
    };
    eframe::run_native(
        "app name?",
        options,
        Box::new(|cc| Box::new(RtApp::new(cc))),
    )
}

#[derive(Default)]
struct RtApp;

impl RtApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for RtApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        egui::SidePanel::left("Render Properties").show(ctx, |ui| {
            ui.heading("Render Properties");
            ui.label("")
        });
    }
}
