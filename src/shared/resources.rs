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
}
