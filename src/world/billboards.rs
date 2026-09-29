//! Roadside billboards showing random images from `assets/billboard-images`.

use bevy::prelude::*;
use rand::Rng;
use rand::seq::IndexedRandom;

use crate::shared::constants::{
    BILLBOARD_CHANCE, BILLBOARD_CLEARANCE, BILLBOARD_HEIGHT, BILLBOARD_IMAGES_DIR,
    BILLBOARD_ROAD_GAP, BILLBOARD_TILT, BILLBOARD_WIDTH, ROAD_HALF_WIDTH, SEGMENT_LENGTH,
};
use crate::shared::resources::{SharedMaterials, SharedMeshes};

const IMAGE_EXTENSIONS: [&str; 3] = ["png", "jpg", "jpeg"];

/// Loads every image in the billboard folder as its own panel material.
pub fn load_billboard_materials(
    asset_server: &AssetServer,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Handle<StandardMaterial>> {
    let Some(dir) = crate::shared::assets_dir().map(|assets| assets.join(BILLBOARD_IMAGES_DIR))
    else {
        return Vec::new();
    };
    let Ok(entries) = std::fs::read_dir(&dir) else {
        info!("assets/{BILLBOARD_IMAGES_DIR} not found; no billboards");
        return Vec::new();
    };

    let mut files: Vec<String> = entries
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file())
        .filter_map(|entry| entry.file_name().into_string().ok())
        .filter(|name| {
            std::path::Path::new(name)
                .extension()
                .and_then(|ext| ext.to_str())
                .is_some_and(|ext| IMAGE_EXTENSIONS.contains(&ext.to_ascii_lowercase().as_str()))
        })
        .collect();
    files.sort();
    info!("Loaded {} billboard image(s) from assets/{BILLBOARD_IMAGES_DIR}", files.len());

    files
        .into_iter()
        .map(|name| {
            let image: Handle<Image> = asset_server.load(format!("{BILLBOARD_IMAGES_DIR}/{name}"));
            materials.add(StandardMaterial {
                base_color_texture: Some(image.clone()),
                // Self-lit a little, like a real backlit billboard, so it reads at night.
                emissive_texture: Some(image),
                emissive: LinearRgba::rgb(0.35, 0.35, 0.35),
                perceptual_roughness: 0.6,
                ..default()
            })
        })
        .collect()
}

pub fn billboard_meshes(meshes: &mut Assets<Mesh>) -> (Handle<Mesh>, Handle<Mesh>, Handle<Mesh>) {
    let face = meshes.add(Rectangle::new(BILLBOARD_WIDTH, BILLBOARD_HEIGHT));
    let frame = meshes.add(Cuboid::new(BILLBOARD_WIDTH + 0.4, BILLBOARD_HEIGHT + 0.4, 0.2));
    let post_height = BILLBOARD_CLEARANCE + BILLBOARD_HEIGHT * 0.5;
    let post = meshes.add(Cylinder::new(0.15, post_height));
    (face, frame, post)
}

/// Maybe adds a billboard as a child of a road segment, on a random side.
pub fn maybe_spawn_billboard(
    commands: &mut Commands,
    meshes: &SharedMeshes,
    materials: &SharedMaterials,
    segment: Entity,
    rng: &mut impl Rng,
) {
    if !rng.random_bool(BILLBOARD_CHANCE) {
        return;
    }
    let Some(image) = materials.billboard_images.choose(rng) else {
        return;
    };

    let side = if rng.random_bool(0.5) { -1.0 } else { 1.0 };
    let x = side * (ROAD_HALF_WIDTH + BILLBOARD_ROAD_GAP);
    let local_z = rng.random_range(-SEGMENT_LENGTH * 0.3..SEGMENT_LENGTH * 0.3);
    // Rectangle faces +Z; turning it around makes it face the oncoming car (-Z)
    // without mirroring the image.
    let yaw = std::f32::consts::PI + side * BILLBOARD_TILT;

    let panel_y = BILLBOARD_CLEARANCE + BILLBOARD_HEIGHT * 0.5;
    let post_height = panel_y;
    let post_x = BILLBOARD_WIDTH * 0.35;

    let billboard = commands
        .spawn((
            Transform::from_xyz(x, 0.0, local_z).with_rotation(Quat::from_rotation_y(yaw)),
            Visibility::default(),
            children![
                (
                    Mesh3d(meshes.billboard_face.clone()),
                    MeshMaterial3d(image.clone()),
                    Transform::from_xyz(0.0, panel_y, 0.11),
                ),
                (
                    Mesh3d(meshes.billboard_frame.clone()),
                    MeshMaterial3d(materials.billboard_frame.clone()),
                    Transform::from_xyz(0.0, panel_y, 0.0),
                ),
                (
                    Mesh3d(meshes.billboard_post.clone()),
                    MeshMaterial3d(materials.billboard_frame.clone()),
                    Transform::from_xyz(-post_x, post_height * 0.5, -0.2),
                ),
                (
                    Mesh3d(meshes.billboard_post.clone()),
                    MeshMaterial3d(materials.billboard_frame.clone()),
                    Transform::from_xyz(post_x, post_height * 0.5, -0.2),
                ),
            ],
        ))
        .id();
    commands.entity(segment).add_child(billboard);
}
