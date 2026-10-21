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

    pub fn save_to_file(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let toml_str = toml::to_string_pretty(self)?;
        std::fs::write(path, toml_str)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roundtrip_toml() {
        let original = AbilitiesDoc::DEFAULT_TOML;
        let doc = AbilitiesDoc::load_from_str(original).expect("Failed to load DEFAULT_TOML");
        let serialized = toml::to_string_pretty(&doc).expect("Failed to serialize");
        let doc2 = AbilitiesDoc::load_from_str(&serialized).expect("Failed to reload serialized TOML");
        assert_eq!(doc.character.name, doc2.character.name);
        assert_eq!(doc.abilities.len(), doc2.abilities.len());
        assert_eq!(doc.combat.armor_class, doc2.combat.armor_class);
    }
}
