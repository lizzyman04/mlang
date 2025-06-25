pub mod docs;
pub mod editor;
pub mod layout;
pub mod symbols;
pub mod tabs;
pub mod toolbar;

use docs::viewer::DocsViewer;
use editor::editor::Editor;
use layout::layout::AppLayout;
use symbols::panel::SymbolPanel;
use tabs::tabbed::TabbedState;
use toolbar::toolbar::ToolBar;

pub struct AppState {
    editor: Editor,
    toolbar: ToolBar,
    tabs: TabbedState,
    symbols: SymbolPanel,
    docs: DocsViewer,
    output: String,
    error: String,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            editor: Editor::default(),
            toolbar: ToolBar::default(),
            tabs: TabbedState::default(),
            symbols: SymbolPanel::default(),
            docs: DocsViewer::new(),
            output: String::from(""),
            error: String::from(""),
        }
    }
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        let mut layout = AppLayout::default();
        layout.render(ctx, frame, self);
    }
}
