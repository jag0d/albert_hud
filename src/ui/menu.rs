#![allow(dead_code)]

use crate::app::{ActivePanel, App};
use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
    Frame,
};

const FRAME_BG: Color = Color::Rgb(24, 28, 42);
const FRAME_BORDER: Color = Color::Rgb(218, 165, 32);  // Gold border matching STATS
const TITLE_FG: Color = Color::Rgb(255, 223, 128);
const SELECTED_BG: Color = Color::Rgb(42, 50, 78);
const TEXT_PRIMARY: Color = Color::Rgb(245, 245, 245);
const TEXT_MUTED: Color = Color::Rgb(156, 163, 175);

pub const MENU_ITEMS: &[&str] = &[
    "Skills",
    "Saving Throws",
    "Equipment",
    "F&T",
];

pub fn render(f: &mut Frame, area: Rect, app: &App) {
    // Currently hardcoded to 0 for initial preview until selection state is added to App
    let selected_index = 0;
    render_with_selection(f, area, app, selected_index);
}

pub fn render_with_selection(f: &mut Frame, area: Rect, _app: &App, selected_index: usize) {
    let mut frame_border: BorderType = BorderType::Rounded;
    if _app.active_panel == ActivePanel::Menu {
        frame_border = BorderType::Thick;
    };
    
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(frame_border)
        .border_style(Style::default().fg(FRAME_BORDER))
        .style(Style::default().bg(FRAME_BG))
        .title(
            Line::from(vec![
                Span::styled(" 📋 ", Style::default().fg(FRAME_BORDER)),
                Span::styled(
                    "MENU",
                    Style::default().fg(TITLE_FG).add_modifier(Modifier::BOLD),
                ),
                Span::styled(" 📋 ", Style::default().fg(FRAME_BORDER)),
            ])
        )
        .title_alignment(Alignment::Center)
        .padding(Padding::horizontal(1));

    let mut lines = Vec::new();
    for (i, &item) in MENU_ITEMS.iter().enumerate() {
        let is_selected = i == selected_index;
        let prefix = if is_selected { " ▶ " } else { "   " };

        let row_style = if is_selected {
            Style::default()
                .bg(SELECTED_BG)
                .fg(TEXT_PRIMARY)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().bg(FRAME_BG).fg(TEXT_PRIMARY)
        };

        let line = Line::from(vec![
            Span::styled(
                prefix,
                if is_selected {
                    Style::default().fg(TITLE_FG)
                } else {
                    Style::default().fg(TEXT_MUTED)
                },
            ),
            Span::styled(
                item,
                if is_selected {
                    Style::default().fg(Color::White).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(TEXT_PRIMARY)
                },
            ),
        ])
        .style(row_style);

        lines.push(line);
        // Spacing line between entries
        //lines.push(Line::from(""));
    }

    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(paragraph, area);
}
