use super::list::default_symbols;
use crate::gui::editor::editor::Editor;
use eframe::egui::{Button, Color32, Grid, Ui, Vec2};

#[derive(Default)]
pub struct SymbolPanel;

impl SymbolPanel {
    pub fn ui(&mut self, ui: &mut Ui, editor: &mut Editor) {
        ui.visuals_mut().panel_fill = Color32::from_rgb(20, 20, 20);
        ui.add_space(5.0);
        ui.label("🧮 Symbols");
        ui.add_space(5.0);
        ui.separator();
        ui.add_space(5.0);

        let available_width = ui.available_width();
        let button_size = 30.0;
        let spacing = 8.0;
        let columns = ((available_width + spacing) / (button_size + spacing)).floor() as usize;
        let columns = columns.max(1).min(4);

        ui.style_mut().spacing.item_spacing = Vec2::new(spacing, spacing);

        Grid::new("symbols_grid")
            .num_columns(columns)
            .spacing([spacing, spacing])
            .show(ui, |ui| {
                for (i, sym) in default_symbols().iter().enumerate() {
                    let button = Button::new(sym.symbol.to_string())
                        .fill(Color32::from_rgb(60, 60, 60))
                        .min_size(Vec2::new(button_size, button_size));
                    if ui.add(button).on_hover_text(sym.label).clicked() {
                        editor.insert_symbol(sym.symbol);
                    }
                    if (i + 1) % columns == 0 {
                        ui.end_row();
                    }
                }
            });
    }
}
