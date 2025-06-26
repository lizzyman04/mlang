use eframe::egui::{Color32, FontFamily, FontId, ScrollArea, Ui};

pub fn error_ui(ui: &mut Ui, error: &str) {
    ui.visuals_mut().panel_fill = Color32::from_rgb(20, 20, 20);
    ui.add_space(5.0);
    ScrollArea::vertical()
        .id_salt("errors_scroll")
        .max_height(ui.available_height())
        .max_width(ui.available_width())
        .show(ui, |ui| {
            ui.visuals_mut().panel_fill = Color32::from_rgb(15, 15, 15);
            ui.add(
                eframe::egui::TextEdit::multiline(&mut error.to_string())
                    .font(FontId::new(14.0, FontFamily::Monospace))
                    .text_color(Color32::from_rgb(255, 100, 100))
                    .desired_width(ui.available_width())
                    .desired_rows((ui.available_height() / 14.0).max(1.0) as usize)
                    .interactive(false),
            );
        });
}
