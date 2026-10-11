pub mod footer;
pub mod header;
pub mod inspector;
pub mod stats;

use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

pub fn render(f: &mut Frame, app: &App) {
    let size = f.area();
    let main_area = centered_rect(88, 92, size);

    // Vertical split:
    // 1. Character Information header (yellow frame)
    // 2. Middle content: STATS & INSPECTOR side by side
    // 3. Bottom controls footer
    let v_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5), // Character Information
            Constraint::Min(10),   // Content panels
            Constraint::Length(2), // Controls footer
        ])
        .split(main_area);

    // 1. Header
    header::render(f, v_chunks[0], &app.abilities_doc.character);

    // 2. Middle: Horizontal split for STATS & INSPECTOR
    let h_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(24), // STATS panel (compact)
            Constraint::Min(30),    // INSPECTOR panel (takes all remaining space)
        ])
        .split(v_chunks[1]);

    stats::render(f, h_chunks[0], app);
    inspector::render(f, h_chunks[1], app);

    // 3. Footer
    footer::render(f, v_chunks[2], app);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
