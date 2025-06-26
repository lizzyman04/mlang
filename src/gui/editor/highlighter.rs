use crate::core::lexer::rules::is_keyword;
use eframe::egui::text::LayoutJob;
use eframe::egui::{Color32, FontId, TextFormat};

pub fn highlight_code(code: &str) -> LayoutJob {
    let mut job = LayoutJob::default();
    let font = FontId::new(14.0, eframe::egui::FontFamily::Monospace);

    let mut current_word = String::new();
    let mut in_string = false;
    let mut in_comment = false;
    let mut current_pos = 0;

    for c in code.chars().chain(std::iter::once('\n')) {
        current_pos += 1;

        if in_comment {
            if c == '\n' {
                in_comment = false;
                job.append(
                    &current_word,
                    0.0,
                    TextFormat {
                        font_id: font.clone(),
                        color: Color32::from_rgb(0, 128, 0),
                        ..Default::default()
                    },
                );
                current_word.clear();
                job.append(
                    "\n",
                    0.0,
                    TextFormat {
                        font_id: font.clone(),
                        color: Color32::WHITE,
                        ..Default::default()
                    },
                );
            } else {
                current_word.push(c);
            }
            continue;
        }

        if in_string {
            current_word.push(c);
            if c == '"' {
                in_string = false;
                job.append(
                    &current_word,
                    0.0,
                    TextFormat {
                        font_id: font.clone(),
                        color: Color32::from_rgb(255, 165, 0),
                        ..Default::default()
                    },
                );
                current_word.clear();
            }
            continue;
        }

        if c == '"' {
            if !current_word.is_empty() {
                append_token(
                    &mut job,
                    &current_word,
                    &font,
                    current_pos - current_word.len(),
                );
                current_word.clear();
            }
            in_string = true;
            current_word.push(c);
            continue;
        }

        if c == '#' && !current_word.is_empty() && current_word.ends_with('#') {
            current_word.pop();
            if !current_word.is_empty() {
                append_token(
                    &mut job,
                    &current_word,
                    &font,
                    current_pos - current_word.len(),
                );
            }
            current_word.clear();
            in_comment = true;
            current_word.push_str("##");
            continue;
        }

        if c.is_whitespace()
            || matches!(
                c,
                '{' | '}' | '(' | ')' | ';' | '+' | '-' | '*' | '/' | '=' | '<' | '>' | '!'
            )
        {
            if !current_word.is_empty() {
                append_token(
                    &mut job,
                    &current_word,
                    &font,
                    current_pos - current_word.len(),
                );
                current_word.clear();
            }
            let symbol_str = c.to_string();
            let color = match c {
                '+' | '-' | '*' | '/' => Color32::from_rgb(200, 200, 200),
                '{' | '}' | '(' | ')' | ';' => Color32::from_rgb(200, 200, 200),
                '<' | '>' | '=' | '!' => Color32::from_rgb(200, 200, 200),
                _ => Color32::WHITE,
            };
            job.append(
                &symbol_str,
                0.0,
                TextFormat {
                    font_id: font.clone(),
                    color,
                    ..Default::default()
                },
            );
            continue;
        }

        current_word.push(c);
    }

    if !current_word.is_empty() && !in_comment && !in_string {
        append_token(
            &mut job,
            &current_word,
            &font,
            current_pos - current_word.len(),
        );
    }

    job
}

fn append_token(job: &mut LayoutJob, word: &str, font: &FontId, _pos: usize) {
    let color = if is_keyword(word) {
        Color32::from_rgb(0, 191, 255)
    } else if word == "true" || word == "false" {
        Color32::from_rgb(50, 205, 50)
    } else if word.chars().all(|c| c.is_numeric() || c == '.') {
        if word.contains('.') {
            Color32::from_rgb(255, 255, 102)
        } else {
            Color32::from_rgb(255, 215, 0)
        }
    } else {
        Color32::WHITE
    };

    job.append(
        word,
        0.0,
        TextFormat {
            font_id: font.clone(),
            color,
            ..Default::default()
        },
    );
}
