//! Car spawning and driving systems.

use bevy::prelude::*;

use crate::shared::components::{
    Car, CarHeadlight, CarHeadlightLens, CarModel, CarModelLamp, HeadlightMode,
};
use crate::shared::constants::{
    CAR_ACCEL, CAR_BRAKE, CAR_MODEL_FRONT_Z, CAR_MODEL_HEADLIGHT_X, CAR_MODEL_LAMP_LONG,
    CAR_MODEL_LAMP_SHORT, CAR_MODEL_OFFSET, CAR_MODEL_PATH, CAR_MODEL_SCALE, CAR_MODEL_YAW,
    CAR_SPEED_MAX, CAR_SPEED_MIN, CAR_STEER, HEADLIGHT_LONG_AIM_Y,
    HEADLIGHT_LONG_AIM_Z, HEADLIGHT_LONG_INNER, HEADLIGHT_LONG_INTENSITY, HEADLIGHT_LONG_OUTER,
    HEADLIGHT_LONG_RANGE, HEADLIGHT_SHORT_AIM_Y, HEADLIGHT_SHORT_AIM_Z, HEADLIGHT_SHORT_INNER,
    HEADLIGHT_SHORT_INTENSITY, HEADLIGHT_SHORT_OUTER, HEADLIGHT_SHORT_RANGE, KMH_TO_WORLD,
    ROAD_HALF_WIDTH, SCORE_REFERENCE_SPEED,
};
use crate::shared::resources::{Difficulty, GameState};

fn car_model_exists() -> bool {
    crate::shared::assets_dir().is_some_and(|assets| assets.join(CAR_MODEL_PATH).exists())
}

pub fn spawn_car(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    asset_server: &AssetServer,
) {
    let use_model = car_model_exists();
    if use_model {
        info!("Using car model assets/{CAR_MODEL_PATH}");
    } else {
        info!("assets/{CAR_MODEL_PATH} not found; using the procedural car");
    }
    let body_visibility = if use_model {
        Visibility::Hidden
    } else {
        Visibility::Inherited
    };

    let body = meshes.add(Cuboid::new(2.0, 0.7, 4.0));
    let cabin = meshes.add(Cuboid::new(1.6, 0.65, 1.8));
    let wheel = meshes.add(Cylinder::new(0.4, 0.35));
    let bumper = meshes.add(Cuboid::new(2.1, 0.25, 0.3));
    let lens = meshes.add(Sphere::new(0.18).mesh().uv(8, 6));

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
    let lens_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.85, 0.85, 0.7),
        emissive: LinearRgba::BLACK,
        perceptual_roughness: 0.2,
        ..default()
    });

    let wheel_rot = Quat::from_rotation_z(std::f32::consts::FRAC_PI_2);

    let car = commands
        .spawn((
            Car {
                speed: CAR_SPEED_MIN,
                acceleration: 0.0,
                lights: HeadlightMode::Off,
            },
            Transform::from_xyz(0.0, 0.55, 0.0),
            Visibility::default(),
            children![(
                Transform::default(),
                body_visibility,
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
            )],
        ))
        .id();

    if use_model {
        let model = commands
            .spawn((
                CarModel,
                WorldAssetRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(CAR_MODEL_PATH))),
                Transform::from_translation(CAR_MODEL_OFFSET)
                    .with_rotation(Quat::from_rotation_y(CAR_MODEL_YAW))
                    .with_scale(Vec3::splat(CAR_MODEL_SCALE)),
            ))
            .id();
        commands.entity(car).add_child(model);
    }

    let (beam_x, beam_z) = if use_model {
        (CAR_MODEL_HEADLIGHT_X, CAR_MODEL_FRONT_Z)
    } else {
        (0.7, 2.15)
    };

    for x in [-beam_x, beam_x] {
        let origin = Vec3::new(x, 0.2, beam_z);
        let beam = commands
            .spawn((
                CarHeadlight,
                SpotLight {
                    color: Color::srgb(1.0, 0.96, 0.85),
                    intensity: 0.0,
                    range: HEADLIGHT_SHORT_RANGE,
                    radius: 0.15,
                    inner_angle: HEADLIGHT_SHORT_INNER,
                    outer_angle: HEADLIGHT_SHORT_OUTER,
                    shadow_maps_enabled: false,
                    ..default()
                },
                Transform::from_translation(origin).looking_at(
                    Vec3::new(x * 0.35, HEADLIGHT_SHORT_AIM_Y, HEADLIGHT_SHORT_AIM_Z),
                    Vec3::Y,
                ),
            ))
            .id();

        let lens_entity = commands
            .spawn((
                CarHeadlightLens,
                Mesh3d(lens.clone()),
                MeshMaterial3d(lens_mat.clone()),
                Transform::from_xyz(x, 0.28, 2.2).with_scale(Vec3::new(1.0, 0.75, 0.55)),
                // The glTF car has its own lamp glow, driven via CarModelLamp.
                body_visibility,
            ))
            .id();

        commands.entity(car).add_children(&[beam, lens_entity]);
    }
}

fn model_lamp_emissive(mode: HeadlightMode) -> LinearRgba {
    let strength = match mode {
        HeadlightMode::Off => 0.0,
        HeadlightMode::Short => CAR_MODEL_LAMP_SHORT,
        HeadlightMode::Long => CAR_MODEL_LAMP_LONG,
    };
    LinearRgba::rgb(strength, strength, strength)
}

/// Tags glTF car meshes that carry the baked lamp texture once the scene has spawned,
/// and syncs their glow with the current headlight mode.
pub fn tag_car_model_lamps(
    mut commands: Commands,
    added: Query<(Entity, &MeshMaterial3d<StandardMaterial>), Added<MeshMaterial3d<StandardMaterial>>>,
    parents: Query<&ChildOf>,
    models: Query<(), With<CarModel>>,
    car_query: Query<&Car>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let Ok(car) = car_query.single() else {
        return;
    };

    for (entity, mat_handle) in &added {
        let mut current = entity;
        let mut in_model = false;
        while let Ok(child_of) = parents.get(current) {
            current = child_of.parent();
            if models.contains(current) {
                in_model = true;
                break;
            }
        }
        if !in_model {
            continue;
        }

        let has_lamps = materials
            .get(&mat_handle.0)
            .is_some_and(|mat| mat.emissive_texture.is_some());
        if !has_lamps {
            continue;
        }

        commands.entity(entity).insert(CarModelLamp);
        if let Some(mut mat) = materials.get_mut(&mat_handle.0) {
            mat.emissive = model_lamp_emissive(car.lights);
        }
    }
}

pub fn toggle_car_lights(
    keys: Res<ButtonInput<KeyCode>>,
    mut car_query: Query<&mut Car>,
    mut beams: Query<(&mut SpotLight, &mut Transform), With<CarHeadlight>>,
    lenses: Query<&MeshMaterial3d<StandardMaterial>, With<CarHeadlightLens>>,
    model_lamps: Query<&MeshMaterial3d<StandardMaterial>, With<CarModelLamp>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if !keys.just_pressed(KeyCode::KeyL) {
        return;
    }

    let Ok(mut car) = car_query.single_mut() else {
        return;
    };

    car.lights = car.lights.next();
    apply_headlight_mode(car.lights, &mut beams, &lenses, &mut materials);

    let lamp_emissive = model_lamp_emissive(car.lights);
    for mat_handle in &model_lamps {
        if let Some(mut mat) = materials.get_mut(&mat_handle.0) {
            mat.emissive = lamp_emissive;
        }
    }
}

fn apply_headlight_mode(
    mode: HeadlightMode,
    beams: &mut Query<(&mut SpotLight, &mut Transform), With<CarHeadlight>>,
    lenses: &Query<&MeshMaterial3d<StandardMaterial>, With<CarHeadlightLens>>,
    materials: &mut Assets<StandardMaterial>,
) {
    let (intensity, range, inner, outer, aim_y, aim_z, emissive) = match mode {
        HeadlightMode::Off => (
            0.0,
            HEADLIGHT_SHORT_RANGE,
            HEADLIGHT_SHORT_INNER,
            HEADLIGHT_SHORT_OUTER,
            HEADLIGHT_SHORT_AIM_Y,
            HEADLIGHT_SHORT_AIM_Z,
            LinearRgba::BLACK,
        ),
        HeadlightMode::Short => (
            HEADLIGHT_SHORT_INTENSITY,
            HEADLIGHT_SHORT_RANGE,
            HEADLIGHT_SHORT_INNER,
            HEADLIGHT_SHORT_OUTER,
            HEADLIGHT_SHORT_AIM_Y,
            HEADLIGHT_SHORT_AIM_Z,
            LinearRgba::rgb(8.0, 7.5, 5.0),
        ),
        HeadlightMode::Long => (
            HEADLIGHT_LONG_INTENSITY,
            HEADLIGHT_LONG_RANGE,
            HEADLIGHT_LONG_INNER,
            HEADLIGHT_LONG_OUTER,
            HEADLIGHT_LONG_AIM_Y,
            HEADLIGHT_LONG_AIM_Z,
            LinearRgba::rgb(16.0, 14.5, 9.0),
        ),
    };

    for (mut light, mut transform) in beams.iter_mut() {
        light.intensity = intensity;
        light.range = range;
        light.inner_angle = inner;
        light.outer_angle = outer;

        let x = transform.translation.x;
        let origin = transform.translation;
        *transform = Transform::from_translation(origin).looking_at(
            Vec3::new(x * 0.35, aim_y, aim_z),
            Vec3::Y,
        );
    }

    for mat_handle in lenses.iter() {
        if let Some(mut mat) = materials.get_mut(&mat_handle.0) {
            mat.emissive = emissive;
        }
    }
}

pub fn drive_car(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<GameState>,
    difficulty: Res<Difficulty>,
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
    let travelled = world_speed * dt;
    transform.translation.z += travelled;
    state.distance += travelled;
    state.score += travelled * score_multiplier(car.speed, *difficulty);
}

pub fn score_multiplier(speed_kmh: f32, difficulty: Difficulty) -> f32 {
    speed_kmh / SCORE_REFERENCE_SPEED * difficulty.score_scale()
}
