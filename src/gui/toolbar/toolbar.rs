use crate::gui::{editor::editor::Editor, symbols::panel::SymbolPanel};
use eframe::egui::{Button, Color32, Ui, Vec2, Window};

#[derive(Default)]
pub struct ToolBar {
    symbols_popup_open: bool,
    docs_popup_open: bool,
}

impl ToolBar {
    pub fn ui(
        &mut self,
        ui: &mut Ui,
        editor: &mut Editor,
        symbols: &mut SymbolPanel,
        output: &mut String,
        error: &mut String,
    ) {
        ui.horizontal(|ui| {
            ui.visuals_mut().widgets.inactive.bg_fill = Color32::from_rgb(50, 50, 50);
            ui.visuals_mut().widgets.hovered.bg_fill = Color32::from_rgb(80, 80, 80);
            ui.style_mut().spacing.button_padding = Vec2::new(8.0, 4.0);

            if ui.add(Button::new("▶️ Run")).clicked() {
                *output = String::from("Executando código... (em breve)");
                *error = String::from("");
            }

            if ui.add(Button::new("🗑 Clear")).clicked() {
                editor.clear();
                *output = String::from("");
                *error = String::from("");
            }

            if ui.add(Button::new("💾 Save")).clicked() {
                *output = String::from("Salvando arquivo... (em breve)");
            }

            ui.add_space(10.0);

            if ui.add(Button::new("🔣 Symbols")).clicked() {
                self.symbols_popup_open = !self.symbols_popup_open;
            }

            if ui.add(Button::new("📖 Docs")).clicked() {
                self.docs_popup_open = !self.docs_popup_open;
            }
        });

        if self.symbols_popup_open {
            Window::new("🔣 Symbols")
                .collapsible(false)
                .resizable(false)
                .fixed_size(Vec2::new(150.0, 200.0))
                .show(ui.ctx(), |ui| {
                    ui.visuals_mut().panel_fill = Color32::from_rgb(40, 40, 40);
                    symbols.ui(ui, editor);
                });
        }
    }

    pub fn is_docs_popup_open(&self) -> bool {
        self.docs_popup_open
    }
}
