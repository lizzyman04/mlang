use eframe::egui::{
    Color32, FontFamily, FontId, RichText, ScrollArea, TextEdit, TextStyle, Ui, Vec2,
};

#[derive(Default)]
pub struct Editor {
    code: String,
    cursor_pos: usize,
    prev_code: String,
    scroll_offset: f32,
}

impl Editor {
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.visuals_mut().widgets.noninteractive.bg_fill = Color32::from_rgb(15, 15, 15);
        ui.visuals_mut().widgets.inactive.bg_fill = Color32::from_rgb(20, 20, 20);
        ui.visuals_mut().widgets.active.bg_fill = Color32::from_rgb(30, 30, 30);

        let line_count = self.code.lines().count().max(1);
        let scroll_offset = self.scroll_offset.max(0.0);

        ui.with_layout(
            eframe::egui::Layout::left_to_right(eframe::egui::Align::Min).with_cross_justify(true),
            |ui| {
                let line_numbers_response = ScrollArea::vertical()
                    .id_salt("line_numbers")
                    .max_width(40.0)
                    .max_height(f32::INFINITY)
                    .scroll_offset(Vec2::new(0.0, scroll_offset))
                    .show(ui, |ui| {
                        ui.visuals_mut().widgets.noninteractive.bg_fill =
                            Color32::from_rgb(30, 30, 30);
                        ui.style_mut().spacing.item_spacing = Vec2::new(0.0, 0.0);
                        ui.with_layout(
                            eframe::egui::Layout::top_down(eframe::egui::Align::Min),
                            |ui| {
                                for i in 1..=line_count {
                                    ui.label(
                                        RichText::new(format!("{:4}", i))
                                            .text_style(TextStyle::Monospace)
                                            .color(Color32::WHITE)
                                            .background_color(Color32::from_rgb(30, 30, 30)),
                                    );
                                }
                            },
                        );
                    });

                ui.visuals_mut().widgets.inactive.bg_fill = Color32::from_rgb(20, 20, 20);
                let id = ui.make_persistent_id("editor");

                let editor_response = ScrollArea::vertical()
                    .id_salt("editor_text")
                    .scroll_offset(Vec2::new(0.0, scroll_offset))
                    .max_height(f32::INFINITY)
                    .show(ui, |ui| {
                        let text_edit = TextEdit::multiline(&mut self.code)
                            .id(id)
                            .desired_width(f32::INFINITY)
                            .code_editor()
                            .font(FontId::new(14.0, FontFamily::Monospace))
                            .margin(Vec2::new(5.0, 0.0))
                            .desired_rows(line_count);

                        ui.style_mut().spacing.item_spacing = Vec2::new(0.0, 0.0);
                        ui.add_sized(
                            Vec2::new(ui.available_width(), ui.available_height()),
                            text_edit,
                        );
                    });

                self.scroll_offset = line_numbers_response
                    .state
                    .offset
                    .y
                    .max(editor_response.state.offset.y);
            },
        );

        let new_len = self.code.len();
        if new_len != self.prev_code.len() {
            let old_len = self.prev_code.len();

            if new_len > old_len {
                self.cursor_pos += new_len - old_len;
            } else {
                self.cursor_pos = self.cursor_pos.saturating_sub(old_len - new_len);
            }

            self.cursor_pos = self.cursor_pos.min(self.code.len());
            self.prev_code = self.code.clone();
            ui.ctx().request_repaint();
        }
    }

    pub fn clear(&mut self) {
        self.code.clear();
        self.prev_code.clear();
        self.cursor_pos = 0;
        self.scroll_offset = 0.0;
    }

    pub fn insert_symbol(&mut self, symbol: char) {
        if self.cursor_pos <= self.code.len() {
            self.code.insert(self.cursor_pos, symbol);
            self.cursor_pos += symbol.len_utf8();
            self.prev_code = self.code.clone();
        }
    }

    pub fn get_code(&self) -> &str {
        &self.code
    }
}
