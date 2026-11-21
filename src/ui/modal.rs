use crate::app::{App, DamageModalState};
use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph},
    Frame,
};

const MODAL_BG: Color = Color::Rgb(20, 24, 36);
const MODAL_BORDER: Color = Color::Rgb(239, 68, 68); // Crimson red
const TITLE_FG: Color = Color::Rgb(248, 113, 113);
const ACCENT_YELLOW: Color = Color::Yellow;
const ACCENT_CYAN: Color = Color::Rgb(56, 189, 248);
const TEXT_PRIMARY: Color = Color::Rgb(245, 245, 245);
const TEXT_MUTED: Color = Color::Rgb(156, 163, 175);

/// Renders a centered floating modal for entering incoming damage.
/// Designed for future expansion with damage types, resistances, and critical modifiers.
pub fn render_damage_modal(f: &mut Frame, state: &DamageModalState, app: &App) {
    let screen = f.area();
    let modal_width = 46.min(screen.width.saturating_sub(4));
    let modal_height = 11.min(screen.height.saturating_sub(2));

    let area = Rect {
        x: screen.x + (screen.width.saturating_sub(modal_width)) / 2,
        y: screen.y + (screen.height.saturating_sub(modal_height)) / 2,
        width: modal_width,
        height: modal_height,
    };

    // 1. Clear the area under the modal so background elements don't bleed through
    f.render_widget(Clear, area);

    // 2. Modal Frame
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Thick)
        .border_style(Style::default().fg(MODAL_BORDER))
        .style(Style::default().bg(MODAL_BG))
        .title(
            Line::from(vec![
                Span::styled(" 🩸 ", Style::default().fg(MODAL_BORDER)),
                Span::styled(
                    "TAKE DAMAGE",
                    Style::default().fg(TITLE_FG).add_modifier(Modifier::BOLD),
                ),
                Span::styled(" 🩸 ", Style::default().fg(MODAL_BORDER)),
            ])
        )
        .title_alignment(Alignment::Center)
        .padding(Padding::horizontal(1));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let combat = &app.abilities_doc.combat;
    let current_pending = app.total_pending_damage();

    let pending_display = if current_pending > 0 {
        format!("-{} HP", current_pending)
    } else {
        "0 HP".to_string()
    };

    // 3. Modal Content Lines
    let mut lines = Vec::new();

    // Health overview
    lines.push(Line::from(vec![
        Span::styled("HP: ", Style::default().fg(TEXT_MUTED)),
        Span::styled(
            format!("{}/{}", combat.hp_current, combat.hp_max),
            Style::default().fg(TEXT_PRIMARY).add_modifier(Modifier::BOLD),
        ),
        Span::raw("  |  "),
        Span::styled("Temp: ", Style::default().fg(TEXT_MUTED)),
        Span::styled(
            format!("{}", combat.hp_temp),
            Style::default().fg(ACCENT_CYAN).add_modifier(Modifier::BOLD),
        ),
        Span::raw("  |  "),
        Span::styled("Pending: ", Style::default().fg(TEXT_MUTED)),
        Span::styled(
            pending_display,
            Style::default().fg(ACCENT_YELLOW).add_modifier(Modifier::BOLD),
        ),
    ]));

    lines.push(Line::from(""));

    // Input prompt and buffer
    lines.push(Line::from(Span::styled(
        "Enter Damage Amount:",
        Style::default().fg(TEXT_MUTED),
    )));

    let display_buffer = if state.amount_buffer.is_empty() {
        " "
    } else {
        &state.amount_buffer
    };

    lines.push(Line::from(vec![
        Span::styled(" ▶ [ ", Style::default().fg(MODAL_BORDER).add_modifier(Modifier::BOLD)),
        Span::styled(
            display_buffer,
            Style::default().fg(TITLE_FG).add_modifier(Modifier::BOLD),
        ),
        Span::styled("█", Style::default().fg(ACCENT_YELLOW)),
        Span::styled(" ]", Style::default().fg(MODAL_BORDER).add_modifier(Modifier::BOLD)),
    ]));

    lines.push(Line::from(""));

    // Modal action controls
    lines.push(Line::from(vec![
        Span::styled("↵ Add to Buffer", Style::default().fg(ACCENT_YELLOW).add_modifier(Modifier::BOLD)),
        Span::raw("   "),
        Span::styled("Esc Cancel", Style::default().fg(TEXT_MUTED)),
    ]));

    let paragraph = Paragraph::new(lines).alignment(Alignment::Center);
    f.render_widget(paragraph, inner);
}
