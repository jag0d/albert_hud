use crate::app::{ActivePanel, App};
use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
    Frame,
};

const FRAME_BG: Color = Color::Rgb(24, 28, 42);
const FRAME_BORDER: Color = Color::Rgb(218, 165, 32);  // Gold border
const TITLE_FG: Color = Color::Rgb(255, 223, 128);
const SELECTED_BG: Color = Color::Rgb(42, 50, 78);
const STAT_BADGE_BG: Color = Color::Rgb(60, 48, 86);
const TEXT_PRIMARY: Color = Color::Rgb(245, 245, 245);
const TEXT_MUTED: Color = Color::Rgb(156, 163, 175);
const ACCENT_CYAN: Color = Color::Rgb(56, 189, 248);

pub fn render(f: &mut Frame, area: Rect, app: &App) {
    let frame_border = if app.active_panel == ActivePanel::Stats {
        BorderType::Thick
    } else {
        BorderType::Rounded
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(frame_border)
        .border_style(Style::default().fg(FRAME_BORDER))
        .style(Style::default().bg(FRAME_BG))
        .title(
            Line::from(vec![
                Span::styled(" ⚔ ", Style::default().fg(FRAME_BORDER)),
                Span::styled(
                    "STATS",
                    Style::default().fg(TITLE_FG).add_modifier(Modifier::BOLD),
                ),
                Span::styled(" ⚔ ", Style::default().fg(FRAME_BORDER)),
            ])
        )
        .title_alignment(Alignment::Center)
        .padding(Padding::horizontal(1));

    let mut list_lines = Vec::new();
    for (i, ability) in app.abilities_doc.abilities.iter().enumerate() {
        let is_selected = i == app.selected_index;
        let prefix = if is_selected { " ▶ " } else { "   " };
        let mod_str = ability.display_modifier();

        let row_style = if is_selected {
            Style::default().bg(SELECTED_BG).fg(TEXT_PRIMARY).add_modifier(Modifier::BOLD)
        } else {
            Style::default().bg(FRAME_BG).fg(TEXT_PRIMARY)
        };

        let line = Line::from(vec![
            Span::styled(prefix, if is_selected { Style::default().fg(TITLE_FG) } else { Style::default().fg(TEXT_MUTED) }),
            Span::styled(
                format!("{}: ", ability.code),
                Style::default().fg(ACCENT_CYAN).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("{:2} ", ability.score),
                if is_selected { Style::default().fg(Color::White).add_modifier(Modifier::BOLD) } else { Style::default().fg(TEXT_PRIMARY) },
            ),
            Span::styled(
                format!(" [{:>3}] ", mod_str),
                Style::default()
                    .bg(STAT_BADGE_BG)
                    .fg(TITLE_FG)
                    .add_modifier(Modifier::BOLD),
            ),
        ]).style(row_style);

        list_lines.push(line);
        // Spacing line between entries
        list_lines.push(Line::from(""));
    }

    let paragraph = Paragraph::new(list_lines).block(block);
    f.render_widget(paragraph, area);
}
