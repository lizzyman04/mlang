use eframe::egui::{
    CentralPanel, Color32, Context, FontFamily, FontId, SidePanel, TextStyle, TopBottomPanel, Vec2,
    Visuals,
};

use crate::gui::AppState;

pub struct AppLayout {
    editor_height: Option<f32>,
}

impl Default for AppLayout {
    fn default() -> Self {
        Self {
            editor_height: None,
        }
    }
}

impl AppLayout {
    pub fn configure_ui_style(ctx: &Context) {
        let mut visuals = Visuals::dark();
        visuals.panel_fill = Color32::from_rgb(30, 30, 30);
        visuals.widgets.noninteractive.bg_fill = Color32::from_rgb(20, 20, 20);
        visuals.widgets.inactive.bg_fill = Color32::from_rgb(50, 50, 50);
        visuals.widgets.hovered.bg_fill = Color32::from_rgb(70, 70, 70);
        ctx.set_visuals(visuals);

        let mut style = (*ctx.style()).clone();
        style
            .text_styles
            .insert(TextStyle::Body, FontId::new(14.0, FontFamily::Proportional));
        style.text_styles.insert(
            TextStyle::Monospace,
            FontId::new(14.0, FontFamily::Monospace),
        );
        style.text_styles.insert(
            TextStyle::Heading,
            FontId::new(18.0, FontFamily::Proportional),
        );
        ctx.set_style(style);
    }

    pub fn render(&mut self, ctx: &Context, _frame: &mut eframe::Frame, state: &mut AppState) {
        Self::configure_ui_style(ctx);

        TopBottomPanel::top("top_bar")
            .exact_height(40.0)
            .show(ctx, |ui| {
                ui.style_mut().spacing.item_spacing = Vec2::new(8.0, 4.0);
                ui.horizontal(|ui| {
                    ui.add_space(5.0);
                    ui.heading("🔢 MLang IDE");
                    ui.add_space(10.0);
                    state.toolbar.ui(
                        ui,
                        &mut state.editor,
                        &mut state.output,
                        &mut state.error,
                        &mut state.tabs,
                    );
                });
            });

        CentralPanel::default().show(ctx, |ui| {
            ui.visuals_mut().panel_fill = Color32::from_rgb(25, 25, 25);
            ui.add_space(5.0);

            if state.toolbar.is_docs_popup_open() || state.toolbar.is_symbols_popup_open() {
                SidePanel::right("docs_viewer")
                    .resizable(true)
                    .default_width(300.0)
                    .show_inside(ui, |ui| {
                        ui.visuals_mut().panel_fill = Color32::from_rgb(20, 20, 20);
                        if state.toolbar.is_docs_popup_open() {
                            state.docs.ui(ui);
                        } else if state.toolbar.is_symbols_popup_open() {
                            state.symbols.ui(ui, &mut state.editor);
                        }
                    });

                ui.vertical(|ui| {
                    let editor_height = self.editor_height.unwrap_or(ui.available_height() * 0.7);
                    ui.scope(|ui| {
                        ui.set_min_height(editor_height);
                        ui.set_max_height(editor_height);
                        state.editor.ui(ui);
                    });

                    ui.scope(|ui| {
                        state.tabs.ui(ui, &state.output, &state.error);
                    });
                });
            } else {
                ui.vertical(|ui| {
                    let editor_height = self.editor_height.unwrap_or(ui.available_height() * 0.7);
                    ui.scope(|ui| {
                        ui.set_min_height(editor_height);
                        ui.set_max_height(editor_height);
                        state.editor.ui(ui);
                    });

                    ui.scope(|ui| {
                        state.tabs.ui(ui, &state.output, &state.error);
                    });
                });
            }
        });
    }
}
