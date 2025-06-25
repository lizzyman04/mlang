use eframe::egui::{RichText, Ui};

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
        ui.horizontal(|ui| {
            if ui
                .selectable_label(matches!(self.selected, Tab::Output), RichText::new("Output"))
                .clicked()
            {
                self.selected = Tab::Output;
            }
            if ui
                .selectable_label(matches!(self.selected, Tab::Errors), RichText::new("Errors"))
                .clicked()
            {
                self.selected = Tab::Errors;
            }
        });

        ui.separator();

        match self.selected {
            Tab::Output => super::output::output_ui(ui, output),
            Tab::Errors => super::errors::error_ui(ui, error),
        }
    }
}