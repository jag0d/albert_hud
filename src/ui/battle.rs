use crate::app::{ActivePanel, App};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
    Frame,
};

const FRAME_BG: Color = Color::Rgb(24, 28, 42);
const FRAME_BORDER: Color = Color::Rgb(218, 165, 32);  // Gold border
const TITLE_FG: Color = Color::Rgb(255, 223, 128);
const SELECTED_BG: Color = Color::Rgb(42, 50, 78);
const TEXT_MUTED: Color = Color::Rgb(156, 163, 175);
const ACCENT_YELLOW: Color = Color::Yellow;
const ACCENT_CYAN: Color = Color::Rgb(56, 189, 248);

pub fn render(f: &mut Frame, area: Rect, app: &App) {
    let is_panel_active = app.active_panel == ActivePanel::InBattle;
    let frame_border = if is_panel_active {
        BorderType::Thick
    } else {
        BorderType::Rounded
    };

    // First level panel: "IN BATTLE"
    let main_block = Block::default()
        .borders(Borders::ALL)
        .border_type(frame_border)
        .border_style(Style::default().fg(FRAME_BORDER))
        .style(Style::default().bg(FRAME_BG))
        .title(
            Line::from(vec![
                Span::styled(" ⚔ ", Style::default().fg(FRAME_BORDER)),
                Span::styled(
                    "IN BATTLE",
                    Style::default().fg(TITLE_FG).add_modifier(Modifier::BOLD),
                ),
                Span::styled(" ⚔ ", Style::default().fg(FRAME_BORDER)),
            ])
        )
        .title_alignment(Alignment::Center)
        .padding(Padding::horizontal(1));

    let inner_area = main_block.inner(area);
    f.render_widget(main_block, area);

    // Second level subpanels placed in a vertical column
    let sub_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Ratio(1, 4),
            Constraint::Ratio(1, 4),
            Constraint::Ratio(1, 4),
            Constraint::Ratio(1, 4),
        ])
        .split(inner_area);

    let combat = &app.abilities_doc.combat;

    let subpanels = [
        ("Armor Class", format!("{}", combat.armor_class)),
        ("Hit Points", combat.hp_display()),
        ("Speed", combat.speed_display()),
        ("Initiative", combat.initiative_display()),
        
    ];

    for (i, (title, value)) in subpanels.iter().enumerate() {
        let is_selected = i == app.selected_battle_stat();
        render_subpanel(
            f,
            sub_chunks[i],
            title,
            value,
            is_selected,
            is_panel_active,
        );
    }
}

fn render_subpanel(
    f: &mut Frame,
    area: Rect,
    title: &str,
    value: &str,
    is_selected: bool,
    is_panel_active: bool,
) {
    let sub_border_type = if is_selected && is_panel_active {
        BorderType::Thick
    } else {
        BorderType::Rounded
    };

    let sub_border_style = if is_selected && is_panel_active {
        Style::default().fg(ACCENT_YELLOW).add_modifier(Modifier::BOLD)
    } else if is_selected {
        Style::default().fg(TITLE_FG)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let title_style = if is_selected && is_panel_active {
        Style::default().fg(ACCENT_YELLOW).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(TEXT_MUTED)
    };

    let bg_color = if is_selected && is_panel_active {
        SELECTED_BG
    } else {
        FRAME_BG
    };

    let sub_block = Block::default()
        .borders(Borders::ALL)
        .border_type(sub_border_type)
        .border_style(sub_border_style)
        .style(Style::default().bg(bg_color))
        .title(Span::styled(format!(" {} ", title), title_style))
        .title_alignment(Alignment::Center);

    let inner = sub_block.inner(area);
    f.render_widget(sub_block, area);

    // Center value inside the subpanel box
    let val_style = if is_selected && is_panel_active {
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
            .fg(ACCENT_CYAN)
            .add_modifier(Modifier::BOLD)
    };

    let paragraph = Paragraph::new(Line::from(Span::styled(value, val_style)))
        .alignment(Alignment::Center);

    // If there is enough vertical space, place the value on line 1 for visual centering
    let val_area = if inner.height >= 3 {
        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length((inner.height - 1) / 2),
                Constraint::Length(1),
                Constraint::Min(0),
            ])
            .split(inner);
        rows[1]
    } else {
        inner
    };

    f.render_widget(paragraph, val_area);
}
