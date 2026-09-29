//! Dynamic obstacle spawning and collision checks.

use bevy::prelude::*;
use rand::Rng;

use crate::shared::components::{Car, Obstacle};
use crate::shared::constants::{CAR_HALF_EXTENTS, LANE_POSITIONS};
use crate::shared::resources::{
    Difficulty, GameState, ObstacleSpawner, SharedMaterials, SharedMeshes,
};

const SPAWN_AHEAD: f32 = 120.0;
const DESPAWN_BEHIND: f32 = 40.0;

pub fn spawn_obstacles(
    mut commands: Commands,
    car_query: Query<&Transform, With<Car>>,
    meshes: Res<SharedMeshes>,
    materials: Res<SharedMaterials>,
    mut spawner: ResMut<ObstacleSpawner>,
    state: Res<GameState>,
    difficulty: Res<Difficulty>,
) {
    if state.crashed {
        return;
    }

    let Ok(car) = car_query.single() else {
        return;
    };

    let mut rng = rand::rng();

    // Cap work per frame so hitching cannot snowball when catching up.
    let mut spawned = 0;
    while spawner.next_spawn_z < car.translation.z + SPAWN_AHEAD && spawned < 4 {
        let lane = LANE_POSITIONS[rng.random_range(0..LANE_POSITIONS.len())];
        let z = spawner.next_spawn_z;

        let twin = rng.random_bool(difficulty.twin_chance());
        let lanes: Vec<f32> = if twin {
            let mut chosen = LANE_POSITIONS.to_vec();
            chosen.retain(|&x| (x - lane).abs() > 0.1);
            let second = chosen[rng.random_range(0..chosen.len())];
            vec![lane, second]
        } else {
            vec![lane]
        };

        for x in lanes {
            if rng.random_bool(0.55) {
                commands.spawn((
                    Mesh3d(meshes.obstacle_cone.clone()),
                    MeshMaterial3d(materials.cone.clone()),
                    Transform::from_xyz(x, 0.7, z),
                    Obstacle {
                        half_extents: Vec3::new(0.5, 0.7, 0.5),
                    },
                ));
            } else {
                commands.spawn((
                    Mesh3d(meshes.obstacle_box.clone()),
                    MeshMaterial3d(materials.crate_mat.clone()),
                    Transform::from_xyz(x, 0.8, z),
                    Obstacle {
                        half_extents: Vec3::new(0.8, 0.8, 0.8),
                    },
                ));
            }
        }

        // Density ramps with distance; floor keeps the live set bounded.
        let scale = difficulty.obstacle_gap_scale();
        let ramp = (state.distance * 0.00015).min(difficulty.max_density_ramp());
        let gap = rng.random_range(18.0..38.0) * scale * (1.0 - ramp);
        spawner.next_spawn_z += gap.max(12.0 * scale);
        spawned += 1;
    }
}

/// Removes obstacles the car has already passed — always runs, even after a crash.
pub fn despawn_passed_obstacles(
    mut commands: Commands,
    car_query: Query<&Transform, With<Car>>,
    obstacles: Query<(Entity, &Transform), With<Obstacle>>,
) {
    let Ok(car) = car_query.single() else {
        return;
    };

    let cutoff = car.translation.z - DESPAWN_BEHIND;
    for (entity, transform) in &obstacles {
        if transform.translation.z < cutoff {
            commands.entity(entity).despawn();
        }
    }
}

pub fn check_collisions(
    mut state: ResMut<GameState>,
    car_query: Query<&Transform, With<Car>>,
    obstacles: Query<(&Transform, &Obstacle)>,
) {
    if state.crashed {
        return;
    }

    let Ok(car) = car_query.single() else {
        return;
    };

    let car_min = car.translation - CAR_HALF_EXTENTS;
    let car_max = car.translation + CAR_HALF_EXTENTS;

    for (transform, obstacle) in &obstacles {
        let obs_min = transform.translation - obstacle.half_extents;
        let obs_max = transform.translation + obstacle.half_extents;

        let overlap = car_min.x <= obs_max.x
            && car_max.x >= obs_min.x
            && car_min.y <= obs_max.y
            && car_max.y >= obs_min.y
            && car_min.z <= obs_max.z
            && car_max.z >= obs_min.z;

        if overlap {
            state.crashed = true;
            break;
        }
    }
}
