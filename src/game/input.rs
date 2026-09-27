//! Restart and quit input handlers.

use bevy::prelude::*;

use crate::shared::components::{Car, Obstacle, RoadSegment};
use crate::shared::constants::{CAR_SPEED_MIN, SEGMENTS_AHEAD, SEGMENTS_BEHIND};
use crate::shared::resources::{
    GameState, ObstacleSpawner, RoadTracker, SharedMaterials, SharedMeshes,
};
use crate::world::road;

pub fn restart_on_crash(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<GameState>,
    mut road_tracker: ResMut<RoadTracker>,
    mut spawner: ResMut<ObstacleSpawner>,
    mut car_query: Query<&mut Transform, With<Car>>,
    mut car_speed: Query<&mut Car>,
    segments: Query<Entity, With<RoadSegment>>,
    obstacles: Query<Entity, With<Obstacle>>,
    meshes: Res<SharedMeshes>,
    materials: Res<SharedMaterials>,
) {
    if !keys.just_pressed(KeyCode::KeyR) {
        return;
    }

    for entity in &segments {
        commands.entity(entity).despawn();
    }
    for entity in &obstacles {
        commands.entity(entity).despawn();
    }

    *state = GameState::default();
    *spawner = ObstacleSpawner::default();
    road_tracker.oldest_segment = -SEGMENTS_BEHIND;
    road_tracker.next_segment = SEGMENTS_AHEAD;

    for i in -SEGMENTS_BEHIND..SEGMENTS_AHEAD {
        road::spawn_road_segment(&mut commands, &meshes, &materials, i);
    }

    if let Ok(mut transform) = car_query.single_mut() {
        *transform = Transform::from_xyz(0.0, 0.55, 0.0);
    }
    if let Ok(mut car) = car_speed.single_mut() {
        car.speed = CAR_SPEED_MIN;
        car.acceleration = 0.0;
    }
}

pub fn quit_on_escape(keys: Res<ButtonInput<KeyCode>>, mut app_exit: MessageWriter<AppExit>) {
    if keys.just_pressed(KeyCode::Escape) {
        app_exit.write(AppExit::Success);
    }
}
