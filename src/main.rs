#![warn(clippy::all, clippy::pedantic)]            
mod editor;
mod terminal;
mod document;
mod row;
pub use document::Document;
pub use row::Row;
use editor::Editor;
pub use terminal::Terminal;
fn main() {
    Editor::default().run();
}
