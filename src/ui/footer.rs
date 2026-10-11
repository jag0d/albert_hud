use crate::app::App;
use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

const FRAME_BG: Color = Color::Rgb(24, 28, 42);
const TITLE_FG: Color = Color::Rgb(255, 223, 128);
const TEXT_MUTED: Color = Color::Rgb(156, 163, 175);

pub fn render(f: &mut Frame, area: Rect, app: &App) {
    let notes = app
        .abilities_doc
        .notes
        .as_deref()
        .unwrap_or("[q / Esc] Quit  |  [↑/k] Prev  |  [↓/j] Next");

    let footer_block = Block::default()
        .borders(Borders::TOP)
        .border_style(Style::default().fg(Color::DarkGray))
        .style(Style::default().bg(FRAME_BG));

    let footer_text = Line::from(vec![
        Span::styled("Controls: ", Style::default().fg(TITLE_FG).add_modifier(Modifier::BOLD)),
        Span::styled(notes, Style::default().fg(TEXT_MUTED)),
    ]);

    let footer_widget = Paragraph::new(footer_text)
        .block(footer_block)
        .alignment(Alignment::Center);
    f.render_widget(footer_widget, area);
}
