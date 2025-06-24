use egui::Ui;

pub fn error_ui(ui: &mut Ui, error: &str) {
    ui.label("Errors:");
    ui.group(|ui| {
        ui.colored_label(egui::Color32::RED, error);
    });
}
