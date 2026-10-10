use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterInfo {
    #[serde(default = "default_name")]
    pub name: String,
    #[serde(default = "default_player")]
    pub player_name: String,
    #[serde(default)]
    pub race: String,
    #[serde(default)]
    pub class: String,
    #[serde(default)]
    pub subclass: String,
    #[serde(default = "default_level")]
    pub level: u32,
    #[serde(default)]
    pub exp: u32,
    #[serde(default)]
    pub background: String,
    #[serde(default)]
    pub alignment: String,
}

fn default_name() -> String {
    "Unknown Character".to_string()
}

fn default_player() -> String {
    "Unknown Player".to_string()
}

fn default_level() -> u32 {
    1
}

impl Default for CharacterInfo {
    fn default() -> Self {
        Self {
            name: default_name(),
            player_name: default_player(),
            race: "Unknown Race".to_string(),
            class: "Adventurer".to_string(),
            subclass: String::new(),
            level: 1,
            exp: 0,
            background: String::new(),
            alignment: String::new(),
        }
    }
}

impl CharacterInfo {
    /// Calculate standard 5e proficiency bonus based on level
    pub fn proficiency_bonus(&self) -> i32 {
        2 + ((self.level.saturating_sub(1)) / 4) as i32
    }

    /// Formatted class and subclass string (e.g. "Wizard (School of Evocation)")
    pub fn class_display(&self) -> String {
        if self.subclass.is_empty() {
            self.class.clone()
        } else {
            format!("{} ({})", self.class, self.subclass)
        }
    }
}
