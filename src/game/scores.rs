//! Persistent scoreboard: one `name<TAB>score` line per run, best first.

use std::path::PathBuf;

use bevy::prelude::*;

use crate::shared::constants::SCOREBOARD_MAX_ENTRIES;
use crate::shared::resources::{GameState, PlayerName, ScoreEntry, Scoreboard};

const APP_DIR: &str = "bevy-card-rider";
const SCORES_FILE: &str = "scores.tsv";

fn scores_path() -> PathBuf {
    let data_dir = std::env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("APPDATA").map(PathBuf::from))
        .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".local/share")));
    match data_dir {
        Some(dir) => dir.join(APP_DIR).join(SCORES_FILE),
        None => PathBuf::from(SCORES_FILE),
    }
}

impl Scoreboard {
    pub fn load() -> Self {
        let path = scores_path();
        let Ok(contents) = std::fs::read_to_string(&path) else {
            return Self::default();
        };

        let mut entries: Vec<ScoreEntry> = contents
            .lines()
            .filter_map(|line| {
                let (name, score) = line.rsplit_once('\t')?;
                Some(ScoreEntry {
                    name: name.to_string(),
                    score: score.trim().parse().ok()?,
                })
            })
            .collect();
        entries.sort_by(|a, b| b.score.cmp(&a.score));
        entries.truncate(SCOREBOARD_MAX_ENTRIES);
        info!("Loaded {} scores from {}", entries.len(), path.display());

        Self {
            entries,
            last_run: None,
        }
    }

    pub fn save(&self) {
        let path = scores_path();
        if let Some(dir) = path.parent() {
            if let Err(err) = std::fs::create_dir_all(dir) {
                warn!("Could not create {}: {err}", dir.display());
                return;
            }
        }
        let contents: String = self
            .entries
            .iter()
            .map(|entry| format!("{}\t{}\n", entry.name, entry.score))
            .collect();
        if let Err(err) = std::fs::write(&path, contents) {
            warn!("Could not save scores to {}: {err}", path.display());
        }
    }

    /// Inserts after any equal scores so earlier runs keep their rank.
    pub fn insert(&mut self, name: String, score: u32) {
        let index = self.entries.partition_point(|entry| entry.score >= score);
        self.entries.insert(index, ScoreEntry { name, score });
        self.entries.truncate(SCOREBOARD_MAX_ENTRIES);
        self.last_run = (index < self.entries.len()).then_some(index);
    }

    pub fn best(&self) -> Option<u32> {
        self.entries.first().map(|entry| entry.score)
    }
}

pub fn record_score_on_crash(
    mut state: ResMut<GameState>,
    player: Res<PlayerName>,
    mut board: ResMut<Scoreboard>,
) {
    if !state.crashed || state.score_saved {
        return;
    }
    state.score_saved = true;
    board.insert(player.0.clone(), state.score.round() as u32);
    board.save();
}
