//! Endless road car rider — drive a procedural car on an infinite road
//! with dynamically spawned obstacles.
//!
//! Controls:
//!   A / Left  — steer left
//!   D / Right — steer right
//!   W / Up    — accelerate
//!   S / Down  — brake
//!   R         — restart after crash
//!   Esc       — quit

use bevy::prelude::*;
use rand::Rng;

const ROAD_HALF_WIDTH: f32 = 6.0;
const LANE_POSITIONS: [f32; 3] = [-4.0, 0.0, 4.0];
const SEGMENT_LENGTH: f32 = 24.0;
const SEGMENTS_AHEAD: i32 = 8;
const SEGMENTS_BEHIND: i32 = 2;

const CAR_SPEED_MIN: f32 = 12.0;
const CAR_SPEED_MAX: f32 = 55.0;
const CAR_ACCEL: f32 = 18.0;
const CAR_BRAKE: f32 = 28.0;
const CAR_STEER: f32 = 22.0;
const CAR_HALF_EXTENTS: Vec3 = Vec3::new(1.1, 0.6, 2.2);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Car Rider — Infinite Road".into(),
                resolution: (1280, 720).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.45, 0.72, 0.95)))
        .init_resource::<GameState>()
        .init_resource::<RoadTracker>()
        .init_resource::<ObstacleSpawner>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                drive_car,
                follow_camera.after(drive_car),
                maintain_infinite_road.after(drive_car),
                spawn_obstacles.after(drive_car),
                check_collisions.after(drive_car),
                update_hud,
                update_gauges.after(drive_car),
                restart_on_crash,
                quit_on_escape,
            ),
        )
        .run();
}

#[derive(Resource)]
struct GameState {
    score: f32,
    crashed: bool,
    distance: f32,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            score: 0.0,
            crashed: false,
            distance: 0.0,
        }
    }
}

#[derive(Resource)]
struct RoadTracker {
    /// Furthest segment index already spawned (along +Z).
    next_segment: i32,
    /// Oldest segment still alive.
    oldest_segment: i32,
}

impl Default for RoadTracker {
    fn default() -> Self {
        Self {
            next_segment: 0,
            oldest_segment: 0,
        }
    }
}

#[derive(Resource)]
struct ObstacleSpawner {
    next_spawn_z: f32,
}

impl Default for ObstacleSpawner {
    fn default() -> Self {
        Self {
            next_spawn_z: 40.0,
        }
    }
}

#[derive(Resource, Clone)]
struct SharedMeshes {
    road: Handle<Mesh>,
    stripe: Handle<Mesh>,
    shoulder: Handle<Mesh>,
    obstacle_box: Handle<Mesh>,
    obstacle_cone: Handle<Mesh>,
    tree_trunk: Handle<Mesh>,
    tree_top: Handle<Mesh>,
}

#[derive(Resource, Clone)]
struct SharedMaterials {
    asphalt: Handle<StandardMaterial>,
    stripe: Handle<StandardMaterial>,
    grass: Handle<StandardMaterial>,
    cone: Handle<StandardMaterial>,
    crate_mat: Handle<StandardMaterial>,
    bark: Handle<StandardMaterial>,
    foliage: Handle<StandardMaterial>,
}

#[derive(Component)]
struct Car {
    speed: f32,
    /// Instantaneous acceleration (units/s²). Positive = accel, negative = brake.
    acceleration: f32,
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
enum GaugeKind {
    Speed,
    Accel,
}

#[derive(Component)]
struct GaugeFill;

#[derive(Component)]
struct GaugeValueText;

#[derive(Component)]
struct FollowCamera;

#[derive(Component)]
struct RoadSegment {
    index: i32,
}

#[derive(Component)]
struct Obstacle {
    half_extents: Vec3,
}

#[derive(Component)]
struct HudText;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut road: ResMut<RoadTracker>,
) {
    let shared_meshes = SharedMeshes {
        road: meshes.add(Cuboid::new(ROAD_HALF_WIDTH * 2.0, 0.15, SEGMENT_LENGTH)),
        stripe: meshes.add(Cuboid::new(0.25, 0.02, 3.0)),
        shoulder: meshes.add(Cuboid::new(18.0, 0.08, SEGMENT_LENGTH)),
        obstacle_box: meshes.add(Cuboid::new(1.6, 1.6, 1.6)),
        obstacle_cone: meshes.add(Cone::new(0.55, 1.4)),
        tree_trunk: meshes.add(Cylinder::new(0.25, 2.0)),
        tree_top: meshes.add(Cone::new(1.4, 2.8)),
    };

    let shared_materials = SharedMaterials {
        asphalt: materials.add(StandardMaterial {
            base_color: Color::srgb(0.18, 0.18, 0.2),
            perceptual_roughness: 0.95,
            ..default()
        }),
        stripe: materials.add(StandardMaterial {
            base_color: Color::srgb(0.95, 0.9, 0.2),
            unlit: true,
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
    };

    // Initial road segments
    for i in -SEGMENTS_BEHIND..SEGMENTS_AHEAD {
        spawn_road_segment(&mut commands, &shared_meshes, &shared_materials, i);
    }
    road.oldest_segment = -SEGMENTS_BEHIND;
    road.next_segment = SEGMENTS_AHEAD;

    spawn_car(&mut commands, &mut meshes, &mut materials);

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 8.0, -12.0).looking_at(Vec3::new(0.0, 0.5, 8.0), Vec3::Y),
        FollowCamera,
    ));

    commands.spawn((
        DirectionalLight {
            illuminance: 12_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(20.0, 40.0, -10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        Text::new(
            "Bevy Car Rider\nA/D steer  W accelerate  S brake  R restart\nScore: 0",
        ),
        TextFont {
            font_size: FontSize::Px(22.0),
            ..default()
        },
        TextColor(Color::srgb(0.05, 0.05, 0.08)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(16.0),
            left: Val::Px(16.0),
            ..default()
        },
        HudText,
    ));

    spawn_gauges(&mut commands);

    commands.insert_resource(shared_meshes);
    commands.insert_resource(shared_materials);
}

fn spawn_gauges(commands: &mut Commands) {
    let panel = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(20.0),
                bottom: Val::Px(20.0),
                width: Val::Px(280.0),
                padding: UiRect::all(Val::Px(14.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(12.0),
                border: UiRect::all(Val::Px(1.0)),
                border_radius: BorderRadius::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.05, 0.06, 0.08, 0.72)),
            BorderColor::all(Color::srgba(1.0, 1.0, 1.0, 0.15)),
        ))
        .id();

    let speed_row = spawn_gauge_row(
        commands,
        "SPEED",
        GaugeKind::Speed,
        Color::srgb(0.15, 0.75, 0.95),
    );
    let accel_row = spawn_gauge_row(
        commands,
        "ACCEL",
        GaugeKind::Accel,
        Color::srgb(0.95, 0.7, 0.15),
    );

    commands.entity(panel).add_children(&[speed_row, accel_row]);
}

fn spawn_gauge_row(
    commands: &mut Commands,
    label: &str,
    kind: GaugeKind,
    fill_color: Color,
) -> Entity {
    let row = commands
        .spawn(Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(4.0),
            ..default()
        })
        .id();

    let header = commands
        .spawn(Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            ..default()
        })
        .id();

    let label_text = commands
        .spawn((
            Text::new(label),
            TextFont {
                font_size: FontSize::Px(14.0),
                ..default()
            },
            TextColor(Color::srgb(0.85, 0.88, 0.92)),
        ))
        .id();

    let value_text = commands
        .spawn((
            Text::new("0"),
            TextFont {
                font_size: FontSize::Px(16.0),
                ..default()
            },
            TextColor(Color::srgb(1.0, 1.0, 1.0)),
            GaugeValueText,
            kind,
        ))
        .id();

    commands
        .entity(header)
        .add_children(&[label_text, value_text]);

    let track = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(18.0),
                padding: UiRect::all(Val::Px(2.0)),
                overflow: Overflow::clip(),
                border_radius: BorderRadius::all(Val::Px(4.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.12, 0.14, 0.18)),
        ))
        .id();

    let fill = commands
        .spawn((
            Node {
                width: Val::Percent(0.0),
                height: Val::Percent(100.0),
                border_radius: BorderRadius::all(Val::Px(3.0)),
                ..default()
            },
            BackgroundColor(fill_color),
            GaugeFill,
            kind,
        ))
        .id();

    commands.entity(track).add_child(fill);
    commands.entity(row).add_children(&[header, track]);
    row
}

fn spawn_car(
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

fn spawn_road_segment(
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

    // Center dashed lane markers
    for i in 0..4 {
        let local_z = -SEGMENT_LENGTH * 0.5 + 3.0 + i as f32 * 6.0;
        children.push((
            Mesh3d(meshes.stripe.clone()),
            MeshMaterial3d(materials.stripe.clone()),
            Transform::from_xyz(0.0, 0.09, local_z),
        ));
    }

    // Edge lines
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

    // Roadside trees (dynamic scenery)
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
}

fn drive_car(
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
        // Gentle coast toward cruise speed
        let cruise = (CAR_SPEED_MIN + CAR_SPEED_MAX) * 0.45;
        car.speed += (cruise - car.speed) * 0.4 * dt;
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

    transform.translation.x += steer * CAR_STEER * dt;
    transform.translation.x = transform
        .translation
        .x
        .clamp(-ROAD_HALF_WIDTH + 1.2, ROAD_HALF_WIDTH - 1.2);

    // Slight body roll while steering
    let target_roll = -steer * 0.12;
    let (yaw, _pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
    transform.rotation = Quat::from_euler(
        EulerRot::YXZ,
        yaw,
        0.0,
        roll + (target_roll - roll) * 8.0 * dt,
    );

    transform.translation.z += car.speed * dt;
    state.distance += car.speed * dt;
    state.score = state.distance;
}

fn follow_camera(
    car_query: Query<&Transform, (With<Car>, Without<FollowCamera>)>,
    mut cam_query: Query<&mut Transform, With<FollowCamera>>,
) {
    let Ok(car) = car_query.single() else {
        return;
    };
    let Ok(mut cam) = cam_query.single_mut() else {
        return;
    };

    let target = Vec3::new(
        car.translation.x * 0.35,
        car.translation.y + 7.5,
        car.translation.z - 14.0,
    );
    cam.translation = cam.translation.lerp(target, 0.12);
    cam.look_at(
        car.translation + Vec3::new(0.0, 1.0, 10.0),
        Vec3::Y,
    );
}

fn maintain_infinite_road(
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

fn spawn_obstacles(
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

        // Occasionally spawn a pair of obstacles
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
                // Traffic cone
                commands.spawn((
                    Mesh3d(meshes.obstacle_cone.clone()),
                    MeshMaterial3d(materials.cone.clone()),
                    Transform::from_xyz(x, 0.7, z),
                    Obstacle {
                        half_extents: Vec3::new(0.5, 0.7, 0.5),
                    },
                ));
            } else {
                // Crate
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

        // Spacing tightens slightly as score grows
        let gap = rng.random_range(18.0..38.0) * (1.0 - (state.score * 0.00015).min(0.35));
        spawner.next_spawn_z += gap.max(12.0);
    }

    // Despawn obstacles far behind the car
    // (handled via a separate query would be cleaner; do it here with commands deferred)
}

fn check_collisions(
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
        // Clean up far-behind obstacles while we're iterating
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

fn update_hud(
    state: Res<GameState>,
    mut hud: Query<&mut Text, With<HudText>>,
) {
    let Ok(mut text) = hud.single_mut() else {
        return;
    };

    if state.crashed {
        **text = format!(
            "CRASHED!  Score: {:.0}\nPress R to restart\nA/D steer  W accelerate  S brake",
            state.score
        );
    } else {
        **text = format!(
            "Bevy Car Rider\nA/D steer  W accelerate  S brake  R restart\nScore: {:.0}",
            state.score
        );
    }
}

fn update_gauges(
    car_query: Query<&Car>,
    mut fills: Query<(&GaugeKind, &mut Node, &mut BackgroundColor), With<GaugeFill>>,
    mut values: Query<(&GaugeKind, &mut Text), With<GaugeValueText>>,
) {
    let Ok(car) = car_query.single() else {
        return;
    };

    let speed_pct = ((car.speed / CAR_SPEED_MAX) * 100.0).clamp(0.0, 100.0);
    // Map accel from [-CAR_BRAKE, CAR_ACCEL] into 0..100 for the bar
    let accel_range = CAR_BRAKE + CAR_ACCEL;
    let accel_pct = (((car.acceleration + CAR_BRAKE) / accel_range) * 100.0).clamp(0.0, 100.0);

    for (kind, mut node, mut bg) in &mut fills {
        match *kind {
            GaugeKind::Speed => {
                node.width = Val::Percent(speed_pct);
                *bg = BackgroundColor(speed_color(speed_pct));
            }
            GaugeKind::Accel => {
                node.width = Val::Percent(accel_pct);
                *bg = BackgroundColor(accel_color(car.acceleration));
            }
        }
    }

    for (kind, mut text) in &mut values {
        match *kind {
            GaugeKind::Speed => {
                **text = format!("{:.0}", car.speed);
            }
            GaugeKind::Accel => {
                **text = format!("{:+.1}", car.acceleration);
            }
        }
    }
}

fn speed_color(pct: f32) -> Color {
    if pct < 40.0 {
        Color::srgb(0.2, 0.85, 0.45)
    } else if pct < 75.0 {
        Color::srgb(0.15, 0.75, 0.95)
    } else {
        Color::srgb(0.95, 0.35, 0.2)
    }
}

fn accel_color(accel: f32) -> Color {
    if accel > 2.0 {
        Color::srgb(0.25, 0.9, 0.35)
    } else if accel < -2.0 {
        Color::srgb(0.95, 0.3, 0.2)
    } else {
        Color::srgb(0.95, 0.7, 0.15)
    }
}

fn restart_on_crash(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<GameState>,
    mut road: ResMut<RoadTracker>,
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
    road.oldest_segment = -SEGMENTS_BEHIND;
    road.next_segment = SEGMENTS_AHEAD;

    for i in -SEGMENTS_BEHIND..SEGMENTS_AHEAD {
        spawn_road_segment(&mut commands, &meshes, &materials, i);
    }

    if let Ok(mut transform) = car_query.single_mut() {
        *transform = Transform::from_xyz(0.0, 0.55, 0.0);
    }
    if let Ok(mut car) = car_speed.single_mut() {
        car.speed = CAR_SPEED_MIN;
        car.acceleration = 0.0;
    }
}

fn quit_on_escape(
    keys: Res<ButtonInput<KeyCode>>,
    mut app_exit: MessageWriter<AppExit>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        app_exit.write(AppExit::Success);
    }
}
