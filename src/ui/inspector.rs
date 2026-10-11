use crate::app::{App, ActivePanel};
use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Padding, Paragraph, Wrap},
    Frame,
};

const FRAME_BG: Color = Color::Rgb(24, 28, 42);
const FRAME_BORDER: Color = Color::Rgb(56, 189, 248); // Cyan border for inspector
const TITLE_FG: Color = Color::Rgb(56, 189, 248);
const TEXT_PRIMARY: Color = Color::Rgb(245, 245, 245);
const TEXT_MUTED: Color = Color::Rgb(156, 163, 175);
const ACCENT_GOLD: Color = Color::Rgb(255, 223, 128);

pub fn render(f: &mut Frame, area: Rect, app: &App) {
    let mut frame_border: BorderType = BorderType::Rounded;
    if app.active_panel == ActivePanel::Inspector {
        frame_border = BorderType::Thick;
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(frame_border)
        .border_style(Style::default().fg(FRAME_BORDER))
        .style(Style::default().bg(FRAME_BG))
        .title(
            Line::from(vec![
                Span::styled(" 🔍 ", Style::default().fg(FRAME_BORDER)),
                Span::styled(
                    "INSPECTOR",
                    Style::default().fg(TITLE_FG).add_modifier(Modifier::BOLD),
                ),
                Span::styled(" 🔍 ", Style::default().fg(FRAME_BORDER)),
            ])
        )
        .title_alignment(Alignment::Center)
        .padding(Padding::uniform(1));

    if let Some(selected) = app.selected_ability() {
        let lines = vec![
            Line::from(vec![
                Span::styled(&selected.name, Style::default().fg(ACCENT_GOLD).add_modifier(Modifier::BOLD)),
                Span::styled(format!(" ({})", selected.code), Style::default().fg(FRAME_BORDER)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Base Score: ", Style::default().fg(TEXT_MUTED)),
                Span::styled(format!("{}", selected.score), Style::default().fg(TEXT_PRIMARY).add_modifier(Modifier::BOLD)),
                Span::raw("   |   "),
                Span::styled("Modifier: ", Style::default().fg(TEXT_MUTED)),
                Span::styled(selected.display_modifier(), Style::default().fg(ACCENT_GOLD).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(Span::styled("Rules & Usage:", Style::default().fg(TEXT_MUTED))),
            Line::from(Span::styled(&selected.description, Style::default().fg(TEXT_PRIMARY))),
        ];

        let paragraph = Paragraph::new(lines)
            .block(block)
            .wrap(Wrap { trim: true });
        f.render_widget(paragraph, area);
    } else {
        let empty_msg = Paragraph::new("No item selected for inspection.")
            .block(block)
            .style(Style::default().fg(TEXT_MUTED));
        f.render_widget(empty_msg, area);
    }
}
