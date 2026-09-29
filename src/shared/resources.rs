//! ECS resources for game state and shared assets.

use bevy::prelude::*;

/// Top-level flow: type a driver name first, then race.
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GamePhase {
    #[default]
    NameEntry,
    Playing,
}

#[derive(Resource, Default)]
pub struct GameState {
    pub score: f32,
    pub crashed: bool,
    pub distance: f32,
    /// Set once the finished run has been written to the scoreboard.
    pub score_saved: bool,
}

#[derive(Resource, Default)]
pub struct PlayerName(pub String);

#[derive(Clone)]
pub struct ScoreEntry {
    pub name: String,
    pub score: u32,
}

/// Saved runs sorted best-first; persisted by [`crate::game::scores`].
#[derive(Resource, Default)]
pub struct Scoreboard {
    pub entries: Vec<ScoreEntry>,
    /// Index of the most recent run in `entries`, if it made the list.
    pub last_run: Option<usize>,
}

#[derive(Resource)]
pub struct RoadTracker {
    /// Furthest segment index already spawned (along +Z).
    pub next_segment: i32,
    /// Oldest segment still alive.
    pub oldest_segment: i32,
}

impl Default for RoadTracker {
    fn default() -> Self {
        Self {
            next_segment: 0,
            oldest_segment: 0,
        }
    }
}

#[derive(Resource)]
pub struct ObstacleSpawner {
    pub next_spawn_z: f32,
}

impl Default for ObstacleSpawner {
    fn default() -> Self {
        Self {
            next_spawn_z: 40.0,
        }
    }
}

#[derive(Resource, Clone)]
pub struct SharedMeshes {
    pub road: Handle<Mesh>,
    pub stripe: Handle<Mesh>,
    pub shoulder: Handle<Mesh>,
    pub obstacle_box: Handle<Mesh>,
    pub obstacle_cone: Handle<Mesh>,
    pub tree_trunk: Handle<Mesh>,
    pub tree_top: Handle<Mesh>,
    pub billboard_face: Handle<Mesh>,
    pub billboard_frame: Handle<Mesh>,
    pub billboard_post: Handle<Mesh>,
}

#[derive(Resource, Clone)]
pub struct SharedMaterials {
    pub asphalt: Handle<StandardMaterial>,
    pub stripe: Handle<StandardMaterial>,
    pub grass: Handle<StandardMaterial>,
    pub cone: Handle<StandardMaterial>,
    pub crate_mat: Handle<StandardMaterial>,
    pub bark: Handle<StandardMaterial>,
    pub foliage: Handle<StandardMaterial>,
    pub land: Handle<StandardMaterial>,
    pub sky: Handle<StandardMaterial>,
    pub hill: Handle<StandardMaterial>,
    pub billboard_frame: Handle<StandardMaterial>,
    /// One material per image in `assets/billboard-images`; empty means no billboards.
    pub billboard_images: Vec<Handle<StandardMaterial>>,
}

/// Advances through a full day/night loop every [`crate::shared::constants::DAY_NIGHT_PERIOD_SECS`].
#[derive(Resource)]
pub struct DayNightCycle {
    /// Normalized time of day in `[0, 1)`. `0` = dawn, `0.25` = noon, `0.5` = dusk, `0.75` = midnight.
    pub time_of_day: f32,
    pub period_secs: f32,
}

impl Default for DayNightCycle {
    fn default() -> Self {
        Self {
            time_of_day: 0.2, // start mid-morning
            period_secs: crate::shared::constants::DAY_NIGHT_PERIOD_SECS,
        }
    }
}

impl DayNightCycle {
    /// `1` at noon, `0` at midnight — smooth day factor.
    pub fn day_factor(&self) -> f32 {
        let sun_altitude = (self.time_of_day * std::f32::consts::TAU - std::f32::consts::FRAC_PI_2)
            .sin();
        sun_altitude.clamp(0.0, 1.0)
    }
}
