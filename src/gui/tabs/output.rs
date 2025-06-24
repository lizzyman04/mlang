use egui::Ui;

pub fn output_ui(ui: &mut Ui, output: &str) {
    ui.label("Output:");
    ui.group(|ui| {
        ui.monospace(output);
    });
}
