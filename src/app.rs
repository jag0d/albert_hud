use crate::{model::ability::{AbilitiesDoc, AbilityEntry}};

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
}

impl App {
    pub fn new(abilities_doc: AbilitiesDoc) -> Self {
        Self {
            running: true,
            abilities_doc,
            selected_index: 0,
            active_panel: ActivePanel::Stats,
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

    pub fn next_panel(&mut self) {
        self.active_panel = self.active_panel.next();
    }
}
