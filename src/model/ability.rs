use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbilityEntry {
    pub name: String,
    pub code: String,
    pub score: i32,
    #[serde(default)]
    pub modifier: Option<String>,
    #[serde(default)]
    pub description: String,
}

impl AbilityEntry {
    pub fn calc_modifier(&self) -> i32 {
        let val = self.score - 10;
        if val >= 0 {
            val / 2
        } else {
            (val - 1) / 2
        }
    }

    pub fn display_modifier(&self) -> String {
        if let Some(ref m) = self.modifier {
            m.clone()
        } else {
            let m = self.calc_modifier();
            if m >= 0 {
                format!("+{}", m)
            } else {
                format!("{}", m)
            }
        }
    }
}

use crate::model::character::CharacterInfo;
use crate::model::combat::CombatStats;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbilitiesDoc {
    #[serde(default)]
    pub character: CharacterInfo,
    #[serde(default)]
    pub combat: CombatStats,
    #[serde(default = "default_title")]
    pub title: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub notes: Option<String>,
    #[serde(default)]
    pub abilities: Vec<AbilityEntry>,
}

fn default_title() -> String {
    "Stats".to_string()
}

impl AbilitiesDoc {
    pub const DEFAULT_TOML: &'static str = include_str!("../../abilities.toml");

    pub fn load_from_str(s: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(s)
    }

    pub fn load_from_file_or_default(path: &Path) -> Self {
        if let Ok(content) = std::fs::read_to_string(path) {
            if let Ok(doc) = Self::load_from_str(&content) {
                return doc;
            }
        }
        Self::load_from_str(Self::DEFAULT_TOML).unwrap_or_else(|_| Self {
            character: CharacterInfo::default(),
            combat: CombatStats::default(),
            title: "Abilities".to_string(),
            description: "Default Ability Scores".to_string(),
            notes: None,
            abilities: Vec::new(),
        })
    }
}
