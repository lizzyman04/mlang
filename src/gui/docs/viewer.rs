use eframe::egui::{Color32, RichText, ScrollArea, Ui};
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};

#[derive(Default)]
pub struct DocsViewer {
    content: String,
    cache: CommonMarkCache,
}

impl DocsViewer {
    pub fn new() -> Self {
        let mut viewer = Self::default();
        if let Ok(content) = super::loader::load_markdown_file("docs/overview.md") {
            viewer.content = content;
        } else {
            viewer.content = "# Error\nUnable to load documentation.".to_string();
        }
        viewer
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        ui.visuals_mut().panel_fill = Color32::from_rgb(40, 40, 40);
        ScrollArea::vertical().show(ui, |ui| {
            ui.vertical(|ui| {
                ui.label(RichText::new("📖 MLang Docs").strong().size(16.0));
                ui.add_space(5.0);
                ui.separator();
                CommonMarkViewer::new().show(ui, &mut self.cache, &self.content);
            });
        });
    }
}
