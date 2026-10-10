pub mod abilities;
pub mod header;

use crate::app::App;
use ratatui::Frame;

pub fn render(f: &mut Frame, app: &App) {
    abilities::render(f, app);
}
