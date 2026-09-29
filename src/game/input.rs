//! Run reset, driver change, and quit input handlers.

use bevy::prelude::*;

use crate::shared::components::{Car, Obstacle, RoadSegment};
use crate::shared::constants::{CAR_SPEED_MIN, SEGMENTS_AHEAD, SEGMENTS_BEHIND};
use crate::shared::resources::{
    GamePhase, GameState, ObstacleSpawner, RoadTracker, SharedMaterials, SharedMeshes,
};
use crate::world::road;

/// Puts the car back at the start of a fresh road. Runs on R and whenever a race starts.
pub fn reset_run(
    mut commands: Commands,
    mut state: ResMut<GameState>,
    mut road_tracker: ResMut<RoadTracker>,
    mut spawner: ResMut<ObstacleSpawner>,
    mut car_query: Query<(&mut Transform, &mut Car)>,
    segments: Query<Entity, With<RoadSegment>>,
    obstacles: Query<Entity, With<Obstacle>>,
    meshes: Res<SharedMeshes>,
    materials: Res<SharedMaterials>,
) {
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

    if let Ok((mut transform, mut car)) = car_query.single_mut() {
        *transform = Transform::from_xyz(0.0, 0.55, 0.0);
        car.speed = CAR_SPEED_MIN;
        car.acceleration = 0.0;
        // Keep headlight mode as the player left it.
    }
}

pub fn change_driver_on_crash(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<GameState>,
    mut next_phase: ResMut<NextState<GamePhase>>,
) {
    if state.crashed && keys.just_pressed(KeyCode::KeyN) {
        next_phase.set(GamePhase::NameEntry);
    }
}

pub fn quit_on_escape(keys: Res<ButtonInput<KeyCode>>, mut app_exit: MessageWriter<AppExit>) {
    if keys.just_pressed(KeyCode::Escape) {
        app_exit.write(AppExit::Success);
    }
}
