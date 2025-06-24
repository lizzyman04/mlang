use egui::{TextEdit, Ui};

pub struct TextEditor {
    pub code: String,
}

impl Default for TextEditor {
    fn default() -> Self {
        Self {
            code: String::from("// Write your MLang code here..."),
        }
    }
}

impl TextEditor {
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.label("Editor:");
        ui.add(
            TextEdit::multiline(&mut self.code)
                .desired_rows(20)
                .code_editor()
                .lock_focus(true)
                .font(egui::TextStyle::Monospace),
        );
    }
}
