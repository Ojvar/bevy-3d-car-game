//! Dynamic obstacle spawning and collision checks.

use bevy::prelude::*;
use rand::Rng;

use crate::shared::components::{Car, Obstacle};
use crate::shared::constants::{CAR_HALF_EXTENTS, LANE_POSITIONS};
use crate::shared::resources::{GameState, ObstacleSpawner, SharedMaterials, SharedMeshes};

pub fn spawn_obstacles(
    mut commands: Commands,
    car_query: Query<&Transform, With<Car>>,
    meshes: Res<SharedMeshes>,
    materials: Res<SharedMaterials>,
    mut spawner: ResMut<ObstacleSpawner>,
    state: Res<GameState>,
) {
    if state.crashed {
        return;
    }

    let Ok(car) = car_query.single() else {
        return;
    };

    let mut rng = rand::rng();

    while spawner.next_spawn_z < car.translation.z + 120.0 {
        let lane = LANE_POSITIONS[rng.random_range(0..LANE_POSITIONS.len())];
        let z = spawner.next_spawn_z;

        let twin = rng.random_bool(0.25);
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

        let gap = rng.random_range(18.0..38.0) * (1.0 - (state.score * 0.00015).min(0.35));
        spawner.next_spawn_z += gap.max(12.0);
    }
}

pub fn check_collisions(
    mut commands: Commands,
    mut state: ResMut<GameState>,
    car_query: Query<&Transform, With<Car>>,
    obstacles: Query<(Entity, &Transform, &Obstacle)>,
) {
    if state.crashed {
        return;
    }

    let Ok(car) = car_query.single() else {
        return;
    };

    let car_min = car.translation - CAR_HALF_EXTENTS;
    let car_max = car.translation + CAR_HALF_EXTENTS;

    for (entity, transform, obstacle) in &obstacles {
        if transform.translation.z < car.translation.z - 30.0 {
            commands.entity(entity).despawn();
            continue;
        }

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
