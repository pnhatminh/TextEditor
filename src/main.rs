#![warn(clippy::all, clippy::pedantic)]            
mod editor;
mod terminal;
mod document;
mod row;
pub use document::Document;
pub use row::Row;
pub use terminal::Terminal;
pub use editor::Position;
use editor::Editor;
fn main() {
    Editor::default().run();
}
