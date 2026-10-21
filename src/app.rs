use std::path::PathBuf;
use crate::{model::{ability::{AbilitiesDoc, AbilityEntry}}, ui};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditableField {
    AbilityScore(usize),
    ArmorClass,
    Initiative,
    Speed,
    HpCurrent,
    HpMax,
    HpTemp,
}

impl EditableField {
    pub fn get_value(&self, doc: &AbilitiesDoc) -> Option<i32> {
        match self {
            Self::AbilityScore(idx) => doc.abilities.get(*idx).map(|a| a.score),
            Self::ArmorClass => Some(doc.combat.armor_class),
            Self::Initiative => Some(doc.combat.initiative),
            Self::Speed => Some(doc.combat.speed),
            Self::HpCurrent => Some(doc.combat.hp_current),
            Self::HpMax => Some(doc.combat.hp_max),
            Self::HpTemp => Some(doc.combat.hp_temp),
        }
    }

    pub fn set_value(&self, doc: &mut AbilitiesDoc, val: i32) {
        match self {
            Self::AbilityScore(idx) => {
                if let Some(ability) = doc.abilities.get_mut(*idx) {
                    ability.score = val;
                    let m = ability.calc_modifier();
                    ability.modifier = Some(if m >= 0 {
                        format!("+{}", m)
                    } else {
                        format!("{}", m)
                    });
                }
            }
            Self::ArmorClass => doc.combat.armor_class = val,
            Self::Initiative => doc.combat.initiative = val,
            Self::Speed => doc.combat.speed = val,
            Self::HpCurrent => doc.combat.hp_current = val,
            Self::HpMax => doc.combat.hp_max = val,
            Self::HpTemp => doc.combat.hp_temp = val,
        }
    }

    pub fn label(&self, doc: &AbilitiesDoc) -> String {
        match self {
            Self::AbilityScore(idx) => doc
                .abilities
                .get(*idx)
                .map(|a| format!("{} Base Score", a.name))
                .unwrap_or_else(|| "Base Score".to_string()),
            Self::ArmorClass => "Armor Class".to_string(),
            Self::Initiative => "Initiative".to_string(),
            Self::Speed => "Speed".to_string(),
            Self::HpCurrent => "Current HP".to_string(),
            Self::HpMax => "Max HP".to_string(),
            Self::HpTemp => "Temp HP".to_string(),
        }
    }

    pub fn matches(&self, target_panel: ActivePanel, app: &App) -> bool {
        match (self, target_panel) {
            (EditableField::AbilityScore(idx), ActivePanel::Stats) => *idx == app.selected_index,
            (EditableField::ArmorClass, ActivePanel::InBattle) => app.selected_battle_index == 0,
            (EditableField::Initiative, ActivePanel::InBattle) => app.selected_battle_index == 1,
            (EditableField::Speed, ActivePanel::InBattle) => app.selected_battle_index == 2,
            (
                EditableField::HpCurrent | EditableField::HpMax | EditableField::HpTemp,
                ActivePanel::InBattle,
            ) => app.selected_battle_index == 3,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    Editing {
        field: EditableField,
        buffer: String,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivePanel {
    Stats,
    Menu,
    Inspector,
    InBattle,
}

impl ActivePanel {
    pub fn next(&self) -> Self {
        match self {
            Self::Stats => Self::Menu,
            Self::Menu => Self::Inspector,
            Self::Inspector => Self::InBattle,
            Self::InBattle => Self::Stats,
        }
    }
}

pub struct App {
    pub running: bool,
    pub abilities_doc: AbilitiesDoc,
    pub file_path: PathBuf,
    pub selected_index: usize,
    pub active_panel: ActivePanel,
    pub prev_active_panel: ActivePanel,
    pub selected_menu_index: usize,
    pub selected_battle_index: usize,
    pub selected_inspector_action: usize,
    pub input_mode: InputMode,
}

impl App {
    pub fn new(abilities_doc: AbilitiesDoc, file_path: impl Into<PathBuf>) -> Self {
        Self {
            running: true,
            abilities_doc,
            file_path: file_path.into(),
            selected_index: 0,
            active_panel: ActivePanel::Stats,
            prev_active_panel: ActivePanel::Stats,
            selected_menu_index: 0,
            selected_battle_index: 0,
            selected_inspector_action: 0,
            input_mode: InputMode::Normal,
        }
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn next_ability(&mut self) {
        self.selected_index = cycle_next(self.selected_index, self.abilities_doc.abilities.len());
    }

    pub fn previous_ability(&mut self) {
        self.selected_index = cycle_prev(self.selected_index, self.abilities_doc.abilities.len());
    }

    pub fn selected_ability(&self) -> Option<&AbilityEntry> {
        self.abilities_doc.abilities.get(self.selected_index)
    }

    pub fn next_menu_item(&mut self) {
        self.selected_menu_index = cycle_next(self.selected_menu_index, ui::menu::MENU_ITEMS.len());
    }

    pub fn previous_menu_item(&mut self) {
        self.selected_menu_index = cycle_prev(self.selected_menu_index, ui::menu::MENU_ITEMS.len());
    }

    pub fn selected_menu_item(&self) -> usize {
        self.selected_menu_index
    }

    pub const BATTLE_STATS_COUNT: usize = 4;

    pub fn next_battle_stat(&mut self) {
        self.selected_battle_index = cycle_next(self.selected_battle_index, Self::BATTLE_STATS_COUNT);
    }

    pub fn previous_battle_stat(&mut self) {
        self.selected_battle_index = cycle_prev(self.selected_battle_index, Self::BATTLE_STATS_COUNT);
    }

    pub fn selected_battle_stat(&self) -> usize {
        self.selected_battle_index
    }

    pub fn navigate_inspector(&mut self) {
        if self.active_panel != ActivePanel::Inspector {
            self.prev_active_panel = self.active_panel;
            self.active_panel = ActivePanel::Inspector;
            self.selected_inspector_action = 0;
        }
    }

    pub fn inspector_action_count(&self) -> usize {
        let target_panel = match self.active_panel {
            ActivePanel::Inspector => self.prev_active_panel,
            other => other,
        };
        match target_panel {
            ActivePanel::Stats => 3, // Roll Check, Saving Throw, Change Base Score
            ActivePanel::InBattle => match self.selected_battle_index {
                3 => 2, // Hit Points: Take Damage, Heal
                _ => 1,
            },
            ActivePanel::Menu => 1,
            _ => 0,
        }
    }
    pub fn save_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.abilities_doc.save_to_file(&self.file_path)
    }

    pub fn next_inspector_action(&mut self) {
        let count = self.inspector_action_count();
        self.selected_inspector_action = cycle_next(self.selected_inspector_action, count);
    }

    pub fn previous_inspector_action(&mut self) {
        let count = self.inspector_action_count();
        self.selected_inspector_action = cycle_prev(self.selected_inspector_action, count);
    }

    pub fn next_panel(&mut self) {
        let next = self.active_panel.next();
        if next != ActivePanel::Inspector {
            self.prev_active_panel = next;
        } else {
            self.prev_active_panel = self.active_panel;
            self.selected_inspector_action = 0;
        }
        self.active_panel = next;
    }

    pub fn navigate_next(&mut self) {
        match self.active_panel {
            ActivePanel::Stats => self.next_ability(),
            ActivePanel::Menu => self.next_menu_item(),
            ActivePanel::Inspector => self.next_inspector_action(),
            ActivePanel::InBattle => self.next_battle_stat(),
        }
    }

    pub fn navigate_previous(&mut self) {
        match self.active_panel {
            ActivePanel::Stats => self.previous_ability(),
            ActivePanel::Menu => self.previous_menu_item(),
            ActivePanel::Inspector => self.previous_inspector_action(),
            ActivePanel::InBattle => self.previous_battle_stat(),
        }
    }

    pub fn navigate_left(&mut self) {
        if self.active_panel == ActivePanel::Inspector {
            self.previous_inspector_action();
        }
    }

    pub fn navigate_right(&mut self) {
        if self.active_panel == ActivePanel::Inspector {
            self.next_inspector_action();
        }
    }
}

/// Safely cycles to the next index, wrapping around to 0
fn cycle_next(current: usize, count: usize) -> usize {
    if count == 0 {
        0
    } else if current + 1 >= count {
        0
    } else {
        current + 1
    }
}

/// Safely cycles to the previous index, wrapping around to count - 1 without underflow
fn cycle_prev(current: usize, count: usize) -> usize {
    if count == 0 {
        0
    } else if current == 0 {
        count - 1
    } else {
        current - 1
    }
}
