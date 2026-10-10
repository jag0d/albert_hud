use crate::model::ability::{AbilitiesDoc, AbilityEntry};

pub struct App {
    pub running: bool,
    pub abilities_doc: AbilitiesDoc,
    pub selected_index: usize,
}

impl App {
    pub fn new(abilities_doc: AbilitiesDoc) -> Self {
        Self {
            running: true,
            abilities_doc,
            selected_index: 0,
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
}
