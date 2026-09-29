//! Startup: scene, assets, car, road, lights, and UI.

use bevy::pbr::{DistanceFog, FogFalloff};
use bevy::prelude::*;

use crate::car;
use crate::shared::components::FollowCamera;
use crate::shared::constants::{
    ROAD_HALF_WIDTH, SEGMENT_LENGTH, SEGMENTS_AHEAD, SEGMENTS_BEHIND,
};
use crate::shared::resources::{RoadTracker, SharedMaterials, SharedMeshes};
use crate::ui::{gauges, hud, scoreboard};
use crate::world::{billboards, environment, road};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut road_tracker: ResMut<RoadTracker>,
    asset_server: Res<AssetServer>,
) {
    let (land, sky, hill) = environment::create_environment_materials(&mut materials);
    let (billboard_face, billboard_frame, billboard_post) = billboards::billboard_meshes(&mut meshes);
    let billboard_images = billboards::load_billboard_materials(&asset_server, &mut materials);

    let shared_meshes = SharedMeshes {
        road: meshes.add(Cuboid::new(ROAD_HALF_WIDTH * 2.0, 0.15, SEGMENT_LENGTH)),
        stripe: meshes.add(Cuboid::new(0.25, 0.02, 3.0)),
        shoulder: meshes.add(Cuboid::new(18.0, 0.08, SEGMENT_LENGTH)),
        obstacle_box: meshes.add(Cuboid::new(1.6, 1.6, 1.6)),
        obstacle_cone: meshes.add(Cone::new(0.55, 1.4)),
        tree_trunk: meshes.add(Cylinder::new(0.25, 2.0)),
        tree_top: meshes.add(Cone::new(1.4, 2.8)),
        billboard_face,
        billboard_frame,
        billboard_post,
    };

    let shared_materials = SharedMaterials {
        asphalt: materials.add(StandardMaterial {
            base_color: Color::srgb(0.18, 0.18, 0.2),
            perceptual_roughness: 0.95,
            ..default()
        }),
        stripe: materials.add(StandardMaterial {
            base_color: Color::srgb(0.95, 0.9, 0.2),
            // Lit so paint darkens at night and catches headlight beams.
            perceptual_roughness: 0.85,
            ..default()
        }),
        grass: materials.add(StandardMaterial {
            base_color: Color::srgb(0.28, 0.55, 0.22),
            perceptual_roughness: 1.0,
            ..default()
        }),
        cone: materials.add(StandardMaterial {
            base_color: Color::srgb(0.95, 0.35, 0.08),
            ..default()
        }),
        crate_mat: materials.add(StandardMaterial {
            base_color: Color::srgb(0.55, 0.32, 0.15),
            ..default()
        }),
        bark: materials.add(StandardMaterial {
            base_color: Color::srgb(0.35, 0.22, 0.12),
            ..default()
        }),
        foliage: materials.add(StandardMaterial {
            base_color: Color::srgb(0.15, 0.45, 0.18),
            ..default()
        }),
        land,
        sky,
        hill,
        billboard_frame: materials.add(StandardMaterial {
            base_color: Color::srgb(0.32, 0.33, 0.36),
            metallic: 0.6,
            perceptual_roughness: 0.5,
            ..default()
        }),
        billboard_images,
    };

    for i in -SEGMENTS_BEHIND..SEGMENTS_AHEAD {
        road::spawn_road_segment(&mut commands, &shared_meshes, &shared_materials, i);
    }
    road_tracker.oldest_segment = -SEGMENTS_BEHIND;
    road_tracker.next_segment = SEGMENTS_AHEAD;

    car::spawn_car(&mut commands, &mut meshes, &mut materials, &asset_server);

    environment::spawn_environment(
        &mut commands,
        &mut meshes,
        shared_materials.land.clone(),
        shared_materials.sky.clone(),
        shared_materials.hill.clone(),
    );

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 8.0, -12.0).looking_at(Vec3::new(0.0, 0.5, 8.0), Vec3::Y),
        FollowCamera,
        DistanceFog {
            color: Color::srgb(0.45, 0.72, 0.95),
            directional_light_color: Color::srgb(1.0, 0.95, 0.85),
            falloff: FogFalloff::Linear {
                start: 100.0,
                end: 320.0,
            },
            ..default()
        },
    ));

    hud::spawn_hud(&mut commands);
    gauges::spawn_gauges(&mut commands);
    scoreboard::spawn_scoreboard(&mut commands);

    commands.insert_resource(shared_meshes);
    commands.insert_resource(shared_materials);
}
