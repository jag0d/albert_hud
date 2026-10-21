pub mod components;

use crate::app::{ActivePanel, App};
use components::InspectorView;
use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Padding, Paragraph, Wrap},
    Frame,
};

const FRAME_BG: Color = Color::Rgb(24, 28, 42);
const FRAME_BORDER: Color = Color::Rgb(56, 189, 248); // Cyan border
const TITLE_FG: Color = Color::Rgb(56, 189, 248);
const TEXT_MUTED: Color = Color::Rgb(156, 163, 175);

pub fn render(f: &mut Frame, area: Rect, app: &App) {
    let frame_border = if app.active_panel == ActivePanel::Inspector {
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

    let target_panel = match app.active_panel {
        ActivePanel::Inspector => app.prev_active_panel,
        other => other,
    };

    let view = match target_panel {
        ActivePanel::InBattle => build_battle_view(app),
        ActivePanel::Menu => build_menu_view(app),
        _ => build_ability_view(app),
    };

    if let Some(view) = view {
        let lines = view.to_lines();
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

fn build_ability_view(app: &App) -> Option<InspectorView> {
    let ability = app.selected_ability()?;
    let mod_str = ability.display_modifier();
    let is_focused = app.active_panel == ActivePanel::Inspector;
    let sel = app.selected_inspector_action;

    let view = InspectorView::new(&ability.name)
        .badge(&ability.code)
        .stat("Base Score", format!("{}", ability.score))
        .stat_accent("Modifier", &mod_str)
        .action(
            format!("🎲 Roll Check (1d20 {})", mod_str),
            is_focused && sel == 0,
        )
        .action(
            format!("🛡 Roll Saving Throw (1d20 {})", mod_str),
            is_focused && sel == 1,
        )
        .section("Rules & Usage:", &ability.description);

    Some(view)
}

fn build_battle_view(app: &App) -> Option<InspectorView> {
    let combat = &app.abilities_doc.combat;
    let is_focused = app.active_panel == ActivePanel::Inspector;
    let sel = app.selected_inspector_action;

    let view = match app.selected_battle_stat() {
        0 => InspectorView::new("Armor Class")
            .badge("AC")
            .stat("Total AC", format!("{}", combat.armor_class))
            .action("🛡 Adjust AC", is_focused && sel == 0)
            .section(
                "Rules & Combat Mechanics:",
                "An attacker's d20 attack roll must meet or exceed your Armor Class to hit. AC is determined by your equipped armor, shield, and Dexterity modifier.",
            ),
        1 => InspectorView::new("Initiative")
            .badge("Turn Order")
            .stat_accent("Bonus", combat.initiative_display())
            .action(
                format!("⚡ Roll Initiative (1d20 {})", combat.initiative_display()),
                is_focused && sel == 0,
            )
            .section(
                "Rules & Combat Mechanics:",
                "At the start of battle, each combatant rolls 1d20 + Initiative modifier to determine turn order. The highest initiative takes action first.",
            ),
        2 => InspectorView::new("Speed")
            .badge("Tactical Movement")
            .stat("Base Movement", combat.speed_display())
            .action("👟 Dash (Double Movement)", is_focused && sel == 0)
            .section(
                "Rules & Combat Mechanics:",
                "The distance in feet you can move on your turn in combat. Dwarf racial trait ensures your speed is not reduced by heavy armor.",
            ),
        3 => InspectorView::new("Hit Points")
            .badge("HP")
            .stat("Current", format!("{}", combat.hp_current))
            .stat("Max", format!("{}", combat.hp_max))
            .stat_accent("Temp", format!("{}", combat.hp_temp))
            .action_row(vec![
                ("🩹 Take Damage", is_focused && sel == 0),
                ("💚 Heal", is_focused && sel == 1),
            ])
            .section(
                "Rules & Combat Mechanics:",
                "Format: 'temporary current/maximum'. Hit Points represent physical and mental stamina, will to live, and luck. Temporary HP buffers incoming damage before current HP is lost. When current HP drops to 0, you fall unconscious and must make death saves.",
            ),
        _ => return None,
    };

    Some(view)
}

fn build_menu_view(app: &App) -> Option<InspectorView> {
    let item_name = crate::ui::menu::MENU_ITEMS
        .get(app.selected_menu_item())
        .copied()
        .unwrap_or("Menu");

    let is_focused = app.active_panel == ActivePanel::Inspector;
    let sel = app.selected_inspector_action;

    let (badge, desc) = match item_name {
        "Skills" => ("Proficiencies", "List of all 18 core D&D 5e skills with associated ability scores and calculated bonuses."),
        "Saving Throws" => ("Defense", "Defensive rolls to resist spells, poisons, traps, and environmental hazards."),
        "Equipment" => ("Inventory", "Weapons, armor, gear, tools, magical items, and currency."),
        "Features and Traits" => ("Character", "Class archetype features, racial traits, and feats available to Albert."),
        _ => ("Section", "Overview for this section."),
    };

    let view = InspectorView::new(item_name)
        .badge(badge)
        .action("↵ Open Details", is_focused && sel == 0)
        .section("Overview:", desc);

    Some(view)
}
