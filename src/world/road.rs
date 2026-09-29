//! Infinite road segment spawning and recycling.

use bevy::prelude::*;
use rand::Rng;

use super::billboards;
use crate::shared::components::{Car, RoadSegment};
use crate::shared::constants::{
    ROAD_HALF_WIDTH, SEGMENT_LENGTH, SEGMENTS_AHEAD, SEGMENTS_BEHIND,
};
use crate::shared::resources::{RoadTracker, SharedMaterials, SharedMeshes};

pub fn spawn_road_segment(
    commands: &mut Commands,
    meshes: &SharedMeshes,
    materials: &SharedMaterials,
    index: i32,
) {
    let z = index as f32 * SEGMENT_LENGTH + SEGMENT_LENGTH * 0.5;
    let mut rng = rand::rng();

    let mut children = vec![
        (
            Mesh3d(meshes.road.clone()),
            MeshMaterial3d(materials.asphalt.clone()),
            Transform::from_xyz(0.0, 0.0, 0.0),
        ),
        (
            Mesh3d(meshes.shoulder.clone()),
            MeshMaterial3d(materials.grass.clone()),
            Transform::from_xyz(0.0, -0.05, 0.0),
        ),
    ];

    for i in 0..4 {
        let local_z = -SEGMENT_LENGTH * 0.5 + 3.0 + i as f32 * 6.0;
        children.push((
            Mesh3d(meshes.stripe.clone()),
            MeshMaterial3d(materials.stripe.clone()),
            Transform::from_xyz(0.0, 0.09, local_z),
        ));
    }

    for x in [-ROAD_HALF_WIDTH + 0.3, ROAD_HALF_WIDTH - 0.3] {
        children.push((
            Mesh3d(meshes.stripe.clone()),
            MeshMaterial3d(materials.stripe.clone()),
            Transform::from_xyz(x, 0.09, 0.0).with_scale(Vec3::new(0.6, 1.0, SEGMENT_LENGTH / 3.0)),
        ));
    }

    let segment = commands
        .spawn((
            RoadSegment { index },
            Transform::from_xyz(0.0, 0.0, z),
            Visibility::default(),
        ))
        .id();

    for child in children {
        let id = commands.spawn(child).id();
        commands.entity(segment).add_child(id);
    }

    for side in [-1.0_f32, 1.0] {
        if rng.random_bool(0.7) {
            let x = side * (ROAD_HALF_WIDTH + 3.0 + rng.random_range(0.0..6.0));
            let local_z = rng.random_range(-SEGMENT_LENGTH * 0.4..SEGMENT_LENGTH * 0.4);
            let trunk = commands
                .spawn((
                    Mesh3d(meshes.tree_trunk.clone()),
                    MeshMaterial3d(materials.bark.clone()),
                    Transform::from_xyz(x, 1.0, local_z),
                ))
                .id();
            let top = commands
                .spawn((
                    Mesh3d(meshes.tree_top.clone()),
                    MeshMaterial3d(materials.foliage.clone()),
                    Transform::from_xyz(x, 3.2, local_z),
                ))
                .id();
            commands.entity(segment).add_child(trunk);
            commands.entity(segment).add_child(top);
        }
    }

    billboards::maybe_spawn_billboard(commands, meshes, materials, segment, &mut rng);
}

pub fn maintain_infinite_road(
    mut commands: Commands,
    car_query: Query<&Transform, With<Car>>,
    segments: Query<(Entity, &RoadSegment)>,
    meshes: Res<SharedMeshes>,
    materials: Res<SharedMaterials>,
    mut road: ResMut<RoadTracker>,
) {
    let Ok(car) = car_query.single() else {
        return;
    };

    let car_segment = (car.translation.z / SEGMENT_LENGTH).floor() as i32;
    let needed_next = car_segment + SEGMENTS_AHEAD;

    while road.next_segment <= needed_next {
        spawn_road_segment(&mut commands, &meshes, &materials, road.next_segment);
        road.next_segment += 1;
    }

    let despawn_before = car_segment - SEGMENTS_BEHIND;
    for (entity, segment) in &segments {
        if segment.index < despawn_before {
            commands.entity(entity).despawn();
            if segment.index >= road.oldest_segment {
                road.oldest_segment = segment.index + 1;
            }
        }
    }
}
