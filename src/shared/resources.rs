//! ECS resources for game state and shared assets.

use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct GameState {
    pub score: f32,
    pub crashed: bool,
    pub distance: f32,
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
