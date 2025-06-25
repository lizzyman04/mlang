use eframe::egui::{Color32, FontFamily, FontId, RichText, ScrollArea, TextEdit, Ui, Vec2};

#[derive(Default)]
pub struct Editor {
    code: String,
    cursor_pos: usize,
    prev_code: String,
    scroll_offset: f32,
}

impl Editor {
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.set_height(ui.available_height());

            ui.visuals_mut().widgets.noninteractive.bg_fill = Color32::from_rgb(15, 15, 15);
            ui.visuals_mut().widgets.inactive.bg_fill = Color32::from_rgb(20, 20, 20);
            ui.visuals_mut().widgets.active.bg_fill = Color32::from_rgb(30, 30, 30);

            let line_count = self.code.lines().count().max(1);
            let scroll_offset = self.scroll_offset.max(0.0);

            ui.horizontal(|ui| {
                ui.set_height(ui.available_height());

                let line_numbers_response = ScrollArea::vertical()
                    .id_salt("line_numbers")
                    .max_width(40.0)
                    .scroll_offset(Vec2::new(0.0, scroll_offset))
                    .show(ui, |ui| {
                        ui.visuals_mut().widgets.noninteractive.bg_fill =
                            Color32::from_rgb(30, 30, 30);
                        ui.vertical(|ui| {
                            for i in 1..=line_count {
                                ui.label(
                                    RichText::new(format!("{:4}", i))
                                        .font(FontId::new(14.0, FontFamily::Monospace))
                                        .color(Color32::WHITE)
                                        .background_color(Color32::from_rgb(30, 30, 30)),
                                );
                            }
                        });
                    });

                ui.visuals_mut().widgets.inactive.bg_fill = Color32::from_rgb(20, 20, 20);
                let id = ui.make_persistent_id("editor");

                let editor_response = ScrollArea::vertical()
                    .id_salt("editor_text")
                    .scroll_offset(Vec2::new(0.0, scroll_offset))
                    .show(ui, |ui| {
                        let text_edit = TextEdit::multiline(&mut self.code)
                            .id(id)
                            .desired_width(f32::INFINITY)
                            .desired_rows(20)
                            .code_editor()
                            .font(FontId::new(14.0, FontFamily::Monospace))
                            .margin(Vec2::new(5.0, 5.0));

                        let available_height = ui.available_height();
                        ui.add_sized([ui.available_width(), available_height], text_edit);
                    });

                self.scroll_offset = line_numbers_response
                    .state
                    .offset
                    .y
                    .max(editor_response.state.offset.y);

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
            });
        });
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
}
