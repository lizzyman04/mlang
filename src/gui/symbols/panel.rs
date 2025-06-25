use super::list::default_symbols;
use crate::gui::editor::editor::Editor;
use eframe::egui::{Button, Color32, Grid, Ui, Vec2};

#[derive(Default)]
pub struct SymbolPanel;

impl SymbolPanel {
    pub fn ui(&mut self, ui: &mut Ui, editor: &mut Editor) {
        ui.vertical(|ui| {
            ui.label("🧮 Symbols");
            ui.add_space(5.0);
            ui.separator();
            ui.style_mut().spacing.item_spacing = Vec2::new(4.0, 4.0);

            Grid::new("symbols_grid")
                .num_columns(4)
                .spacing([8.0, 8.0])
                .show(ui, |ui| {
                    for sym in default_symbols() {
                        let button = Button::new(sym.symbol.to_string())
                            .fill(Color32::from_rgb(60, 60, 60))
                            .min_size(Vec2::new(30.0, 30.0));
                        if ui.add(button).on_hover_text(sym.label).clicked() {
                            editor.insert_symbol(sym.symbol);
                        }
                    }
                });
        });
    }
}
