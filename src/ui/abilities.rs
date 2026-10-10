use crate::app::App;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Padding, Paragraph, Wrap},
    Frame,
};

// Distinct color palette for the HUD frame
const FRAME_BG: Color = Color::Rgb(24, 28, 42);       // Dark midnight slate (contrasts with default terminal)
const FRAME_BORDER: Color = Color::Rgb(218, 165, 32);  // Classic D&D gold border
const TITLE_FG: Color = Color::Rgb(255, 223, 128);     // Warm gold title
const SELECTED_BG: Color = Color::Rgb(42, 50, 78);     // Highlighted item background
const STAT_BADGE_BG: Color = Color::Rgb(60, 48, 86);   // Deep mystic purple for stat badges
const TEXT_PRIMARY: Color = Color::Rgb(245, 245, 245);
const TEXT_MUTED: Color = Color::Rgb(156, 163, 175);
const ACCENT_CYAN: Color = Color::Rgb(56, 189, 248);

pub fn render(f: &mut Frame, app: &App) {
    let size = f.area();

    // Center the HUD area inside the terminal window
    let main_area = centered_rect(88, 92, size);

    // Split vertically into:
    // 1. Character Information Panel (Yellow Frame)
    // 2. Abilities Panel (Gold Frame)
    let panels = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5), // Character Information Frame
            Constraint::Min(10),   // Abilities Frame
        ])
        .split(main_area);

    // Render the Character Info panel at the top
    crate::ui::header::render(f, panels[0], &app.abilities_doc.character);

    // Render the Abilities panel below it
    render_abilities_panel(f, panels[1], app);
}

fn render_abilities_panel(f: &mut Frame, area: Rect, app: &App) {
    let frame_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(FRAME_BORDER))
        .style(Style::default().bg(FRAME_BG))
        .title(
            Line::from(vec![
                Span::styled(" ⚔ ", Style::default().fg(FRAME_BORDER)),
                Span::styled(
                    &app.abilities_doc.title,
                    Style::default()
                        .fg(TITLE_FG)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(" ⚔ ", Style::default().fg(FRAME_BORDER)),
            ])
        )
        .title_alignment(Alignment::Center)
        .padding(Padding::uniform(1));

    let inner_area = frame_block.inner(area);
    f.render_widget(frame_block, area);

    // Layout inside the abilities panel
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(8),    // Ability score list & detail panel
            Constraint::Length(2), // Footer / Controls from TOML notes
        ])
        .split(inner_area);

    render_abilities(f, chunks[0], app);
    render_footer(f, chunks[1], app);
}

fn render_abilities(f: &mut Frame, area: Rect, app: &App) {
    // Split into Left (List of 6 abilities) and Right (Detailed ability inspector)
    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(24), // Compact column for "STR: 10 [+0]"
            Constraint::Min(30),    // Inspector gets all remaining space
        ])
        .split(area);

    // Left column: Ability score cards
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
        // Small spacing line between entries if room permits
        list_lines.push(Line::from(""));
    }

    let list_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .border_style(Style::default().fg(Color::DarkGray))
        .style(Style::default().bg(FRAME_BG))
        .title(" Ability Scores ")
        .padding(Padding::horizontal(1));

    let list_paragraph = Paragraph::new(list_lines).block(list_block);
    f.render_widget(list_paragraph, body_chunks[0]);

    // Right column: Detailed inspection of selected ability
    let detail_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .border_style(Style::default().fg(Color::DarkGray))
        .style(Style::default().bg(FRAME_BG))
        .title(" Ability Description ")
        .padding(Padding::uniform(1));

    if let Some(selected) = app.selected_ability() {
        let detail_text = vec![
            Line::from(vec![
                Span::styled(&selected.name, Style::default().fg(TITLE_FG).add_modifier(Modifier::BOLD)),
                Span::styled(format!(" ({})", selected.code), Style::default().fg(ACCENT_CYAN)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Base Score: ", Style::default().fg(TEXT_MUTED)),
                Span::styled(format!("{}", selected.score), Style::default().fg(TEXT_PRIMARY).add_modifier(Modifier::BOLD)),
                Span::raw("   |   "),
                Span::styled("Modifier: ", Style::default().fg(TEXT_MUTED)),
                Span::styled(selected.display_modifier(), Style::default().fg(TITLE_FG).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(Span::styled("Rules & Usage (from TOML):", Style::default().fg(TEXT_MUTED))),
            Line::from(Span::styled(&selected.description, Style::default().fg(TEXT_PRIMARY))),
        ];
        let detail_paragraph = Paragraph::new(detail_text)
            .block(detail_block)
            .wrap(Wrap { trim: true });
        f.render_widget(detail_paragraph, body_chunks[1]);
    } else {
        let empty_msg = Paragraph::new("No ability selected")
            .block(detail_block)
            .style(Style::default().fg(TEXT_MUTED));
        f.render_widget(empty_msg, body_chunks[1]);
    }
}

fn render_footer(f: &mut Frame, area: Rect, app: &App) {
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

/// Helper function to create a centered rect inside a given area
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
