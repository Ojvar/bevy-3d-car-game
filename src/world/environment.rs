//! Sky, land, and a continuous day/night cycle (full loop every minute).

use bevy::light::light_consts::lux;
use bevy::pbr::{DistanceFog, FogFalloff};
use bevy::prelude::*;

use crate::shared::components::{
    Car, FollowCamera, GroundFollow, Hill, Moon, SkyDome, Sun,
};
use crate::shared::constants::{GROUND_SIZE, ROAD_HALF_WIDTH, SKY_DOME_RADIUS};
use crate::shared::resources::DayNightCycle;

pub fn create_environment_materials(
    materials: &mut Assets<StandardMaterial>,
) -> (Handle<StandardMaterial>, Handle<StandardMaterial>, Handle<StandardMaterial>) {
    let land = materials.add(StandardMaterial {
        base_color: Color::srgb(0.22, 0.48, 0.18),
        perceptual_roughness: 1.0,
        ..default()
    });
    let sky = materials.add(StandardMaterial {
        base_color: Color::srgb(0.45, 0.72, 0.95),
        unlit: true,
        ..default()
    });
    let hill = materials.add(StandardMaterial {
        base_color: Color::srgb(0.25, 0.42, 0.2),
        perceptual_roughness: 0.95,
        ..default()
    });
    (land, sky, hill)
}

pub fn spawn_environment(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    land: Handle<StandardMaterial>,
    sky: Handle<StandardMaterial>,
    hill: Handle<StandardMaterial>,
) {
    commands.spawn((
        Sun,
        DirectionalLight {
            color: Color::srgb(1.0, 0.95, 0.85),
            illuminance: lux::AMBIENT_DAYLIGHT,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(40.0, 80.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        Moon,
        DirectionalLight {
            color: Color::srgb(0.55, 0.65, 0.95),
            illuminance: 0.0,
            shadow_maps_enabled: false,
            ..default()
        },
        Transform::from_xyz(-30.0, 50.0, -40.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        GroundFollow,
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(GROUND_SIZE * 0.5)))),
        MeshMaterial3d(land),
        Transform::from_xyz(0.0, -0.12, 0.0),
    ));

    let hill_mesh = meshes.add(Cone::new(18.0, 28.0));
    for (i, side) in [-1.0_f32, 1.0].into_iter().enumerate() {
        for j in 0..6 {
            let z_offset = -60.0 + j as f32 * 50.0;
            let lateral_x = side * (ROAD_HALF_WIDTH + 38.0 + ((i + j) as f32 * 5.5) % 18.0);
            let scale = 0.75 + ((i * 3 + j) as f32 * 0.11) % 0.55;
            commands.spawn((
                Hill {
                    lateral_x,
                    z_offset,
                },
                Mesh3d(hill_mesh.clone()),
                MeshMaterial3d(hill.clone()),
                Transform::from_xyz(lateral_x, 0.0, z_offset)
                    .with_scale(Vec3::new(scale, scale * 0.9, scale)),
            ));
        }
    }

    let sky_mesh = Sphere::new(SKY_DOME_RADIUS)
        .mesh()
        .ico(4)
        .expect("sky icosphere");
    commands.spawn((
        SkyDome,
        Mesh3d(meshes.add(sky_mesh)),
        MeshMaterial3d(sky),
        // Negative scale flips normals so the inside of the sphere is visible.
        Transform::from_scale(Vec3::splat(-1.0)),
    ));
}

/// Advances time of day and updates sun, moon, ambient light, fog, and sky color.
pub fn update_day_night(
    time: Res<Time>,
    mut cycle: ResMut<DayNightCycle>,
    mut clear_color: ResMut<ClearColor>,
    mut ambient: ResMut<GlobalAmbientLight>,
    mut sun_query: Query<(&mut Transform, &mut DirectionalLight), (With<Sun>, Without<Moon>)>,
    mut moon_query: Query<(&mut Transform, &mut DirectionalLight), (With<Moon>, Without<Sun>)>,
    sky_query: Query<&MeshMaterial3d<StandardMaterial>, With<SkyDome>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut fog_query: Query<&mut DistanceFog, With<FollowCamera>>,
) {
    cycle.time_of_day = (cycle.time_of_day + time.delta_secs() / cycle.period_secs) % 1.0;

    let t = cycle.time_of_day;
    let angle = t * std::f32::consts::TAU;
    let sun_dir = Vec3::new(angle.cos(), angle.sin(), 0.25).normalize();
    let day = cycle.day_factor();
    let night = 1.0 - day;

    let sky = sky_color(t, day);
    clear_color.0 = sky;

    if let Ok(mat_handle) = sky_query.single() {
        if let Some(mut mat) = materials.get_mut(&mat_handle.0) {
            mat.base_color = sky;
        }
    }

    ambient.color = Color::srgb(0.55 + 0.35 * day, 0.6 + 0.3 * day, 0.75 + 0.1 * day);
    ambient.brightness = 40.0 + 140.0 * day;

    if let Ok((mut transform, mut light)) = sun_query.single_mut() {
        *transform = Transform::from_translation(sun_dir * 100.0).looking_at(Vec3::ZERO, Vec3::Y);
        light.color = sun_color(day);
        light.illuminance = lux::AMBIENT_DAYLIGHT * day.powf(1.4);
    }

    if let Ok((mut transform, mut light)) = moon_query.single_mut() {
        let moon_dir = -sun_dir;
        *transform = Transform::from_translation(moon_dir * 80.0).looking_at(Vec3::ZERO, Vec3::Y);
        light.illuminance = lux::FULL_MOON_NIGHT * night.powf(1.2) * 12.0;
        light.color = Color::srgb(0.55, 0.65, 0.95);
    }

    if let Ok(mut fog) = fog_query.single_mut() {
        fog.color = sky;
        fog.directional_light_color = sun_color(day);
        fog.falloff = FogFalloff::Linear {
            start: 80.0 + 40.0 * day,
            end: 280.0 + 80.0 * day,
        };
    }
}

/// Keeps the sky on the camera and land/hills under the car.
pub fn follow_environment(
    car_query: Query<&Transform, (With<Car>, Without<GroundFollow>, Without<SkyDome>, Without<Hill>)>,
    cam_query: Query<
        &Transform,
        (
            With<FollowCamera>,
            Without<GroundFollow>,
            Without<SkyDome>,
            Without<Hill>,
        ),
    >,
    mut ground_query: Query<&mut Transform, (With<GroundFollow>, Without<SkyDome>, Without<Hill>)>,
    mut hill_query: Query<(&Hill, &mut Transform), (Without<GroundFollow>, Without<SkyDome>)>,
    mut sky_query: Query<&mut Transform, (With<SkyDome>, Without<GroundFollow>, Without<Hill>)>,
) {
    if let Ok(cam) = cam_query.single() {
        for mut sky in &mut sky_query {
            let scale = sky.scale;
            *sky = Transform::from_translation(cam.translation).with_scale(scale);
        }
    }

    let Ok(car) = car_query.single() else {
        return;
    };

    for mut ground in &mut ground_query {
        ground.translation.x = car.translation.x;
        ground.translation.z = car.translation.z;
    }

    for (hill, mut transform) in &mut hill_query {
        let span = 300.0;
        let z = car.translation.z
            + (hill.z_offset - car.translation.z + span * 0.5).rem_euclid(span)
            - span * 0.5;
        transform.translation.x = hill.lateral_x;
        transform.translation.z = z;
    }
}

fn sky_color(time_of_day: f32, day: f32) -> Color {
    let day_sky = Color::srgb(0.45, 0.72, 0.95);
    let night_sky = Color::srgb(0.02, 0.03, 0.08);
    let dawn_dusk = Color::srgb(0.95, 0.45, 0.25);

    let twilight = {
        let d0 = (time_of_day - 0.0).abs().min((time_of_day - 1.0).abs());
        let d1 = (time_of_day - 0.5).abs();
        (1.0 - (d0.min(d1) / 0.08).clamp(0.0, 1.0)).powf(1.5)
    };

    let base = Mix::mix(&day_sky, &night_sky, 1.0 - day);
    Mix::mix(&base, &dawn_dusk, twilight * 0.65)
}

fn sun_color(day: f32) -> Color {
    let noon = Color::srgb(1.0, 0.97, 0.9);
    let sunset = Color::srgb(1.0, 0.55, 0.25);
    Mix::mix(&noon, &sunset, (1.0 - day).clamp(0.0, 1.0).powf(0.7))
}
