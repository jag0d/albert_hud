use crate::{model::{ability::{AbilitiesDoc, AbilityEntry}}, ui};

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
    pub selected_index: usize,
    pub active_panel: ActivePanel,
    pub prev_active_panel: ActivePanel,
    pub selected_menu_index: usize,
    pub selected_battle_index: usize,
}

impl App {
    pub fn new(abilities_doc: AbilitiesDoc) -> Self {
        Self {
            running: true,
            abilities_doc,
            selected_index: 0,
            active_panel: ActivePanel::Stats,
            prev_active_panel: ActivePanel::Stats,
            selected_menu_index: 0,
            selected_battle_index: 0,
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
        }
    }

    pub fn next_panel(&mut self) {
        let next = self.active_panel.next();
        if next != ActivePanel::Inspector {
            self.prev_active_panel = next;
        } else {
            self.prev_active_panel = self.active_panel;
        }
        self.active_panel = next;
    }

    pub fn navigate_next(&mut self) {
        match self.active_panel {
            ActivePanel::Stats => self.next_ability(),
            ActivePanel::Menu => self.next_menu_item(),
            ActivePanel::Inspector => self.navigate_inspector(),
            ActivePanel::InBattle => self.next_battle_stat(),
        }
    }

    pub fn navigate_previous(&mut self) {
        match self.active_panel {
            ActivePanel::Stats => self.previous_ability(),
            ActivePanel::Menu => self.previous_menu_item(),
            ActivePanel::Inspector => self.navigate_inspector(),
            ActivePanel::InBattle => self.previous_battle_stat(),
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
