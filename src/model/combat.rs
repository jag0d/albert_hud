use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatStats {
    #[serde(default = "default_ac")]
    pub armor_class: i32,
    #[serde(default = "default_initiative")]
    pub initiative: i32,
    #[serde(default = "default_speed")]
    pub speed: i32,
    #[serde(default = "default_hp")]
    pub hp_current: i32,
    #[serde(default = "default_hp")]
    pub hp_max: i32,
    #[serde(default)]
    pub hp_temp: i32,
}

fn default_ac() -> i32 {
    18
}

fn default_initiative() -> i32 {
    2
}

fn default_speed() -> i32 {
    25
}

fn default_hp() -> i32 {
    44
}

impl Default for CombatStats {
    fn default() -> Self {
        Self {
            armor_class: default_ac(),
            initiative: default_initiative(),
            speed: default_speed(),
            hp_current: default_hp(),
            hp_max: default_hp(),
            hp_temp: 0,
        }
    }
}

impl CombatStats {
    /// Formats hit points as 'temporary current/maximum'
    pub fn hp_display(&self) -> String {
        format!("{} {}/{}", self.hp_temp, self.hp_current, self.hp_max)
    }

    /// Formats initiative with +/- prefix
    pub fn initiative_display(&self) -> String {
        if self.initiative >= 0 {
            format!("+{}", self.initiative)
        } else {
            format!("{}", self.initiative)
        }
    }

    /// Formats speed
    pub fn speed_display(&self) -> String {
        format!("{} ft", self.speed)
    }
}
