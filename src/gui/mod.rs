// pub mod docs;
// pub mod editor;
// pub mod symbols;
// pub mod tabs;

use eframe::egui::{CentralPanel, Context};

#[derive(Default)]
pub struct AppState;

impl eframe::App for AppState {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("MLang IDE — In development");
        });
    }
}
