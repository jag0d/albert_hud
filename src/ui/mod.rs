pub mod battle;
pub mod footer;
pub mod header;
pub mod inspector;
pub mod menu;
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
    // 2. Middle content: 3 columns (Left, Middle, Right)
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

    // 2. Middle: Horizontal split for LEFT (STATS + MENU), MIDDLE (INSPECTOR), RIGHT (IN BATTLE)
    let h_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(24), // Left column (STATS & MENU)
            Constraint::Min(30),    // Middle column (INSPECTOR - fills remaining space!)
            Constraint::Length(24), // Right column (IN BATTLE - fixed compact space)
        ])
        .split(v_chunks[1]);
    
    // 2.1 Left column: Vertical split for STATS and MENU
    let stat_col = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(14),
            Constraint::Min(0),
        ])
        .split(h_chunks[0]);
    
    stats::render(f, stat_col[0], app);
    menu::render(f, stat_col[1], app);

    // 2.2 Middle column: INSPECTOR fills all remaining width
    inspector::render(f, h_chunks[1], app);

    // 2.3 Right column: IN BATTLE panel
    battle::render(f, h_chunks[2], app);

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
