//! Car spawning and driving systems.

use bevy::prelude::*;

use crate::shared::components::Car;
use crate::shared::constants::{
    CAR_ACCEL, CAR_BRAKE, CAR_SPEED_MAX, CAR_SPEED_MIN, CAR_STEER, KMH_TO_WORLD,
    ROAD_HALF_WIDTH,
};
use crate::shared::resources::GameState;

pub fn spawn_car(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let body = meshes.add(Cuboid::new(2.0, 0.7, 4.0));
    let cabin = meshes.add(Cuboid::new(1.6, 0.65, 1.8));
    let wheel = meshes.add(Cylinder::new(0.4, 0.35));
    let bumper = meshes.add(Cuboid::new(2.1, 0.25, 0.3));

    let paint = materials.add(StandardMaterial {
        base_color: Color::srgb(0.85, 0.12, 0.15),
        metallic: 0.35,
        perceptual_roughness: 0.35,
        ..default()
    });
    let glass = materials.add(StandardMaterial {
        base_color: Color::srgba(0.4, 0.7, 0.95, 0.7),
        alpha_mode: AlphaMode::Blend,
        metallic: 0.1,
        perceptual_roughness: 0.1,
        ..default()
    });
    let rubber = materials.add(StandardMaterial {
        base_color: Color::srgb(0.08, 0.08, 0.08),
        perceptual_roughness: 0.9,
        ..default()
    });
    let chrome = materials.add(StandardMaterial {
        base_color: Color::srgb(0.75, 0.75, 0.8),
        metallic: 0.9,
        perceptual_roughness: 0.2,
        ..default()
    });

    let wheel_rot = Quat::from_rotation_z(std::f32::consts::FRAC_PI_2);

    commands.spawn((
        Car {
            speed: CAR_SPEED_MIN,
            acceleration: 0.0,
        },
        Transform::from_xyz(0.0, 0.55, 0.0),
        Visibility::default(),
        children![
            (
                Mesh3d(body),
                MeshMaterial3d(paint.clone()),
                Transform::from_xyz(0.0, 0.15, 0.0),
            ),
            (
                Mesh3d(cabin),
                MeshMaterial3d(glass),
                Transform::from_xyz(0.0, 0.75, -0.2),
            ),
            (
                Mesh3d(bumper.clone()),
                MeshMaterial3d(chrome.clone()),
                Transform::from_xyz(0.0, 0.05, 2.05),
            ),
            (
                Mesh3d(bumper),
                MeshMaterial3d(chrome),
                Transform::from_xyz(0.0, 0.05, -2.05),
            ),
            (
                Mesh3d(wheel.clone()),
                MeshMaterial3d(rubber.clone()),
                Transform::from_xyz(-1.05, -0.2, 1.2).with_rotation(wheel_rot),
            ),
            (
                Mesh3d(wheel.clone()),
                MeshMaterial3d(rubber.clone()),
                Transform::from_xyz(1.05, -0.2, 1.2).with_rotation(wheel_rot),
            ),
            (
                Mesh3d(wheel.clone()),
                MeshMaterial3d(rubber.clone()),
                Transform::from_xyz(-1.05, -0.2, -1.2).with_rotation(wheel_rot),
            ),
            (
                Mesh3d(wheel),
                MeshMaterial3d(rubber),
                Transform::from_xyz(1.05, -0.2, -1.2).with_rotation(wheel_rot),
            ),
        ],
    ));
}

pub fn drive_car(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<GameState>,
    mut query: Query<(&mut Transform, &mut Car)>,
) {
    let Ok((mut transform, mut car)) = query.single_mut() else {
        return;
    };

    if state.crashed {
        car.acceleration = 0.0;
        return;
    }

    let dt = time.delta_secs();
    let prev_speed = car.speed;

    if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) {
        car.speed = (car.speed + CAR_ACCEL * dt).min(CAR_SPEED_MAX);
    } else if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) {
        car.speed = (car.speed - CAR_BRAKE * dt).max(CAR_SPEED_MIN * 0.35);
    } else {
        // Coast toward a mid cruise speed (km/h).
        let cruise = (CAR_SPEED_MIN + CAR_SPEED_MAX) * 0.45;
        car.speed += (cruise - car.speed) * 0.4 * dt;
        car.speed = car.speed.max(CAR_SPEED_MIN);
    }

    car.acceleration = if dt > f32::EPSILON {
        (car.speed - prev_speed) / dt
    } else {
        0.0
    };

    let mut steer = 0.0;
    if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
        steer -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
        steer += 1.0;
    }

    // Lateral steer scales lightly with forward speed so high km/h still feels controllable.
    let steer_scale = (car.speed * KMH_TO_WORLD / 30.0).clamp(0.55, 1.35);
    transform.translation.x += steer * CAR_STEER * steer_scale * dt;
    transform.translation.x = transform
        .translation
        .x
        .clamp(-ROAD_HALF_WIDTH + 1.2, ROAD_HALF_WIDTH - 1.2);

    let target_roll = -steer * 0.12;
    let (yaw, _pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
    transform.rotation = Quat::from_euler(
        EulerRot::YXZ,
        yaw,
        0.0,
        roll + (target_roll - roll) * 8.0 * dt,
    );

    let world_speed = car.speed * KMH_TO_WORLD;
    transform.translation.z += world_speed * dt;
    state.distance += world_speed * dt;
    state.score = state.distance;
}
