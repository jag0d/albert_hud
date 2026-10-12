use crate::{model::{ability::{AbilitiesDoc, AbilityEntry}}, ui};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivePanel {
    Stats,
    Menu,
    Inspector, 
}

impl ActivePanel {
    pub fn next(&self) -> Self {
        match self {
            Self::Inspector => Self::Stats,
            Self::Stats => Self::Menu,
            Self::Menu => Self::Inspector,
        }
    }
}

pub struct App {
    pub running: bool,
    pub abilities_doc: AbilitiesDoc,
    pub selected_index: usize,
    pub active_panel: ActivePanel,
    pub selected_menu_index: usize,
}

impl App {
    pub fn new(abilities_doc: AbilitiesDoc) -> Self {
        Self {
            running: true,
            abilities_doc,
            selected_index: 0,
            active_panel: ActivePanel::Stats,
            selected_menu_index: 0,
        }
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn next_ability(&mut self) {
        let count = self.abilities_doc.abilities.len();
        if count > 0 {
            if self.selected_index + 1 >= count {
                self.selected_index = 0;
            } else {
                self.selected_index += 1;
            }
        }
    }

    pub fn previous_ability(&mut self) {
        let count = self.abilities_doc.abilities.len();
        if count > 0 {
            if self.selected_index == 0 {
                self.selected_index = count - 1;
            } else {
                self.selected_index -= 1;
            }
        }
    }

    pub fn selected_ability(&self) -> Option<&AbilityEntry> {
        self.abilities_doc.abilities.get(self.selected_index)
    }

    pub fn next_menu_item(&mut self) {
        let count = ui::menu::MENU_ITEMS.len();
        if count > 0 {
            if self.selected_menu_index + 1 >= count {
                self.selected_menu_index = 0;
            } else {
                self.selected_menu_index += 1;
            }
        }
    }

    pub fn previous_menu_item(&mut self) {
        let count = ui::menu::MENU_ITEMS.len();
        if count > 0 {
            if self.selected_menu_index == 0 {
                self.selected_menu_index = count - 1;
            } else {
                self.selected_menu_index -= 1;
            }
        }
    }

    pub fn selected_menu_item(&self) -> usize {
        self.selected_menu_index
    }

    pub fn scroll_inspector(&mut self) {
        //only a placeholder until implemented
        self.next_ability();
    }

    pub fn next_panel(&mut self) {
        self.active_panel = self.active_panel.next();
    }

    pub fn navigate_next(&mut self) {
        match self.active_panel {
            ActivePanel::Stats => self.next_ability(),
            ActivePanel::Menu => self.next_menu_item(),
            ActivePanel::Inspector => self.scroll_inspector(),
        }
    }

    pub fn navigate_previous(&mut self) {
        match self.active_panel {
            ActivePanel::Stats => self.previous_ability(),
            ActivePanel::Menu => self.previous_menu_item(),
            ActivePanel::Inspector => self.scroll_inspector(),
        }
    }
}
