use eframe::egui::{Color32, RichText, ScrollArea, Ui};

#[derive(Default)]
pub enum Tab {
    #[default]
    Output,
    Errors,
}

#[derive(Default)]
pub struct TabbedState {
    pub selected: Tab,
}

impl TabbedState {
    pub fn ui(&mut self, ui: &mut Ui, output: &str, error: &str) {
        ui.visuals_mut().widgets.inactive.bg_fill = Color32::from_rgb(50, 50, 50);
        ui.visuals_mut().widgets.hovered.bg_fill = Color32::from_rgb(80, 80, 80);
        ui.visuals_mut().widgets.active.bg_fill = Color32::from_rgb(70, 70, 70);
        ui.style_mut().spacing.button_padding = eframe::egui::Vec2::new(8.0, 4.0);

        ui.horizontal(|ui| {
            ui.add_space(5.0);
            if ui
                .selectable_label(
                    matches!(self.selected, Tab::Output),
                    RichText::new("📜 Output").size(14.0).strong(),
                )
                .clicked()
            {
                self.selected = Tab::Output;
            }
            if ui
                .selectable_label(
                    matches!(self.selected, Tab::Errors),
                    RichText::new("⚠️ Errors").size(14.0).strong(),
                )
                .clicked()
            {
                self.selected = Tab::Errors;
            }
            ui.add_space(5.0);
        });

        ui.add_space(5.0);
        ui.separator();

        ScrollArea::vertical()
            .id_salt("tab_content")
            .max_height(ui.available_height())
            .max_width(ui.available_width())
            .show(ui, |ui| {
                ui.visuals_mut().panel_fill = Color32::from_rgb(50, 50, 50);
                match self.selected {
                    Tab::Output => super::output::output_ui(ui, output),
                    Tab::Errors => super::errors::error_ui(ui, error),
                }
            });
    }
}
