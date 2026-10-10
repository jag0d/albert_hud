use crate::model::character::CharacterInfo;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
    Frame,
};

const FRAME_BG: Color = Color::Rgb(24, 28, 42);
const TEXT_MUTED: Color = Color::Rgb(156, 163, 175);
const TEXT_PRIMARY: Color = Color::White;
const ACCENT_CYAN: Color = Color::Rgb(56, 189, 248);
const ACCENT_YELLOW: Color = Color::Yellow;

pub fn render(f: &mut Frame, area: Rect, character: &CharacterInfo) {
    // Yellow bordered frame for the character info panel
    let header_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(ACCENT_YELLOW))
        .style(Style::default().bg(FRAME_BG))
        .title(
            Line::from(vec![
                Span::styled(" 🧙 ", Style::default().fg(ACCENT_YELLOW)),
                Span::styled(
                    "Character Information",
                    Style::default()
                        .fg(ACCENT_YELLOW)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(" 🧙 ", Style::default().fg(ACCENT_YELLOW)),
            ])
        )
        .title_alignment(Alignment::Center)
        .padding(Padding::horizontal(1));

    let inner_area = header_block.inner(area);
    f.render_widget(header_block, area);

    // Split inner area into 3 horizontal columns
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(34),
            Constraint::Percentage(36),
            Constraint::Percentage(30),
        ])
        .split(inner_area);

    // Column 1: Identity & Background
    let col1_lines = vec![
        Line::from(vec![
            Span::styled("Name: ", Style::default().fg(TEXT_MUTED)),
            Span::styled(
                &character.name,
                Style::default().fg(TEXT_PRIMARY).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("Race: ", Style::default().fg(TEXT_MUTED)),
            Span::styled(
                &character.race,
                Style::default().fg(TEXT_PRIMARY),
            ),
        ]),
        Line::from(vec![
            Span::styled("Background: ", Style::default().fg(TEXT_MUTED)),
            Span::styled(
                &character.background,
                Style::default().fg(TEXT_PRIMARY),
            ),
        ]),
    ];
    let col1_widget = Paragraph::new(col1_lines).style(Style::default().bg(FRAME_BG));
    f.render_widget(col1_widget, cols[0]);

    // Column 2: Class, Subclass, Level, Alignment
    let col2_lines = vec![
        Line::from(vec![
            Span::styled("Class: ", Style::default().fg(TEXT_MUTED)),
            Span::styled(
                character.class_display(),
                Style::default().fg(ACCENT_CYAN).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("Level: ", Style::default().fg(TEXT_MUTED)),
            Span::styled(
                format!("{}", character.level),
                Style::default().fg(ACCENT_YELLOW).add_modifier(Modifier::BOLD),
            ),
            Span::raw("   "),
            Span::styled("Prof: ", Style::default().fg(TEXT_MUTED)),
            Span::styled(
                format!("+{}", character.proficiency_bonus()),
                Style::default().fg(ACCENT_YELLOW).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("Alignment: ", Style::default().fg(TEXT_MUTED)),
            Span::styled(
                &character.alignment,
                Style::default().fg(TEXT_PRIMARY),
            ),
        ]),
    ];
    let col2_widget = Paragraph::new(col2_lines).style(Style::default().bg(FRAME_BG));
    f.render_widget(col2_widget, cols[1]);

    // Column 3: Player & EXP
    let col3_lines = vec![
        Line::from(vec![
            Span::styled("Player: ", Style::default().fg(TEXT_MUTED)),
            Span::styled(
                &character.player_name,
                Style::default().fg(TEXT_PRIMARY).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("EXP: ", Style::default().fg(TEXT_MUTED)),
            Span::styled(
                format!("{}", character.exp),
                Style::default().fg(TEXT_PRIMARY),
            ),
        ]),
    ];
    let col3_widget = Paragraph::new(col3_lines).style(Style::default().bg(FRAME_BG));
    f.render_widget(col3_widget, cols[2]);
}
