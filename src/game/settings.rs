//! Persistent player settings: `key=value` lines in `settings.txt`.

use bevy::prelude::*;

use crate::shared::resources::Difficulty;

const SETTINGS_FILE: &str = "settings.txt";

impl Difficulty {
    pub fn load() -> Self {
        let path = crate::shared::data_file(SETTINGS_FILE);
        let Ok(contents) = std::fs::read_to_string(&path) else {
            return Self::default();
        };
        contents
            .lines()
            .filter_map(|line| line.split_once('='))
            .find(|(key, _)| key.trim() == "difficulty")
            .and_then(|(_, value)| Self::from_key(value.trim()))
            .unwrap_or_default()
    }

    pub fn save(self) {
        let path = crate::shared::data_file(SETTINGS_FILE);
        if let Some(dir) = path.parent()
            && let Err(err) = std::fs::create_dir_all(dir)
        {
            warn!("Could not create {}: {err}", dir.display());
            return;
        }
        if let Err(err) = std::fs::write(&path, format!("difficulty={}\n", self.key())) {
            warn!("Could not save settings to {}: {err}", path.display());
        }
    }
}

pub fn save_settings(difficulty: Res<Difficulty>) {
    difficulty.save();
}
