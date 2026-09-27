//! Circular dashboard gauges (speedometer + accel dial).

use bevy::prelude::*;

use crate::shared::components::{Car, GaugeKind, GaugeNeedle, GaugeValueText};
use crate::shared::constants::{
    CAR_ACCEL, GAUGE_ACCEL_MAX, GAUGE_ACCEL_MIN, GAUGE_NEEDLE_LEN, GAUGE_SIZE, GAUGE_SPEED_MAX,
    GAUGE_SPEED_MIN, GAUGE_START_RAD, GAUGE_SWEEP_RAD,
};

pub fn spawn_gauges(commands: &mut Commands) {
    let panel = commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            right: Val::Px(16.0),
            bottom: Val::Px(16.0),
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(18.0),
            align_items: AlignItems::FlexEnd,
            ..default()
        })
        .id();

    let speed = spawn_circular_gauge(
        commands,
        "SPEED",
        "km/h",
        GaugeKind::Speed,
        &["0", "50", "100", "150", "200", "250", "285"],
    );
    let accel = spawn_circular_gauge(
        commands,
        "ACCEL",
        "x1000",
        GaugeKind::Accel,
        &["0", "1", "2", "3", "4", "5", "6", "7", "8"],
    );

    commands.entity(panel).add_children(&[speed, accel]);
}

fn spawn_circular_gauge(
    commands: &mut Commands,
    title: &str,
    unit: &str,
    kind: GaugeKind,
    ticks: &[&str],
) -> Entity {
    let face_size = GAUGE_SIZE - 14.0;

    let root = commands
        .spawn(Node {
            width: Val::Px(GAUGE_SIZE + 24.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Px(6.0),
            ..default()
        })
        .id();

    let bezel = commands
        .spawn((
            Node {
                width: Val::Px(GAUGE_SIZE),
                height: Val::Px(GAUGE_SIZE),
                border: UiRect::all(Val::Px(6.0)),
                border_radius: BorderRadius::MAX,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.18, 0.18, 0.2)),
            BorderColor::all(Color::srgb(0.55, 0.55, 0.58)),
        ))
        .id();

    let face = commands
        .spawn((
            Node {
                width: Val::Px(face_size),
                height: Val::Px(face_size),
                border_radius: BorderRadius::MAX,
                border: UiRect::all(Val::Px(2.0)),
                overflow: Overflow::clip(),
                ..default()
            },
            BackgroundColor(Color::srgb(0.06, 0.07, 0.09)),
            BorderColor::all(Color::srgb(0.25, 0.26, 0.28)),
        ))
        .id();

    let tick_count = ticks.len().max(2);
    for (i, label) in ticks.iter().enumerate() {
        let t = i as f32 / (tick_count - 1) as f32;
        let angle = GAUGE_START_RAD + t * GAUGE_SWEEP_RAD;

        let tick = commands
            .spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px((face_size - 3.0) * 0.5),
                    top: Val::Px((face_size - GAUGE_NEEDLE_LEN) * 0.5),
                    width: Val::Px(3.0),
                    height: Val::Px(GAUGE_NEEDLE_LEN),
                    ..default()
                },
                BackgroundColor(Color::NONE),
                UiTransform::from_rotation(Rot2::radians(angle)),
                children![(
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(0.0),
                        top: Val::Px(6.0),
                        width: Val::Px(3.0),
                        height: Val::Px(14.0),
                        ..default()
                    },
                    BackgroundColor(if t > 0.85 {
                        Color::srgb(0.9, 0.2, 0.15)
                    } else {
                        Color::srgb(0.9, 0.9, 0.92)
                    }),
                )],
            ))
            .id();
        commands.entity(face).add_child(tick);

        if tick_count <= 7 || i % 2 == 0 || i + 1 == tick_count {
            let radius = face_size * 0.34;
            let x = angle.sin() * radius;
            let y = -angle.cos() * radius;
            let num = commands
                .spawn((
                    Text::new(*label),
                    TextFont {
                        font_size: FontSize::Px(11.0),
                        ..default()
                    },
                    TextColor(Color::srgb(0.85, 0.86, 0.88)),
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(face_size * 0.5 + x - 10.0),
                        top: Val::Px(face_size * 0.5 + y - 8.0),
                        ..default()
                    },
                ))
                .id();
            commands.entity(face).add_child(num);
        }
    }

    let needle = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px((face_size - 5.0) * 0.5),
                top: Val::Px((face_size - GAUGE_NEEDLE_LEN) * 0.5),
                width: Val::Px(5.0),
                height: Val::Px(GAUGE_NEEDLE_LEN),
                ..default()
            },
            BackgroundColor(Color::NONE),
            UiTransform::from_rotation(Rot2::radians(GAUGE_START_RAD)),
            GaugeNeedle,
            kind,
            children![(
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(1.0),
                    top: Val::Px(8.0),
                    width: Val::Px(3.0),
                    height: Val::Px(GAUGE_NEEDLE_LEN * 0.48),
                    border_radius: BorderRadius::all(Val::Px(1.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.92, 0.12, 0.12)),
            )],
        ))
        .id();
    commands.entity(face).add_child(needle);

    let hub = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px((face_size - 16.0) * 0.5),
                top: Val::Px((face_size - 16.0) * 0.5),
                width: Val::Px(16.0),
                height: Val::Px(16.0),
                border_radius: BorderRadius::MAX,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.75, 0.75, 0.78)),
            BorderColor::all(Color::srgb(0.35, 0.35, 0.38)),
        ))
        .id();
    commands.entity(face).add_child(hub);

    let readout = commands
        .spawn((
            Text::new("0"),
            TextFont {
                font_size: FontSize::Px(18.0),
                ..default()
            },
            TextColor(Color::srgb(0.95, 0.95, 0.7)),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(face_size * 0.5 - 28.0),
                bottom: Val::Px(28.0),
                width: Val::Px(56.0),
                justify_content: JustifyContent::Center,
                ..default()
            },
            GaugeValueText,
            kind,
        ))
        .id();
    commands.entity(face).add_child(readout);

    let unit_text = commands
        .spawn((
            Text::new(unit),
            TextFont {
                font_size: FontSize::Px(10.0),
                ..default()
            },
            TextColor(Color::srgb(0.55, 0.58, 0.6)),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(face_size * 0.5 - 24.0),
                bottom: Val::Px(14.0),
                width: Val::Px(48.0),
                justify_content: JustifyContent::Center,
                ..default()
            },
        ))
        .id();
    commands.entity(face).add_child(unit_text);

    commands.entity(bezel).add_child(face);

    let title_text = commands
        .spawn((
            Text::new(title),
            TextFont {
                font_size: FontSize::Px(13.0),
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.92)),
        ))
        .id();

    commands.entity(root).add_children(&[bezel, title_text]);
    root
}

pub fn update_gauges(
    car_query: Query<&Car>,
    mut needles: Query<(&GaugeKind, &mut UiTransform), With<GaugeNeedle>>,
    mut values: Query<(&GaugeKind, &mut Text), With<GaugeValueText>>,
) {
    let Ok(car) = car_query.single() else {
        return;
    };

    let speed_value = car.speed.clamp(GAUGE_SPEED_MIN, GAUGE_SPEED_MAX);
    let accel_t = (car.acceleration.max(0.0) / CAR_ACCEL).clamp(0.0, 1.0);
    let accel_value = GAUGE_ACCEL_MIN + accel_t * (GAUGE_ACCEL_MAX - GAUGE_ACCEL_MIN);

    for (kind, mut transform) in &mut needles {
        let t = match *kind {
            GaugeKind::Speed => {
                (speed_value - GAUGE_SPEED_MIN) / (GAUGE_SPEED_MAX - GAUGE_SPEED_MIN)
            }
            GaugeKind::Accel => {
                (accel_value - GAUGE_ACCEL_MIN) / (GAUGE_ACCEL_MAX - GAUGE_ACCEL_MIN)
            }
        };
        transform.rotation = Rot2::radians(GAUGE_START_RAD + t.clamp(0.0, 1.0) * GAUGE_SWEEP_RAD);
    }

    for (kind, mut text) in &mut values {
        match *kind {
            GaugeKind::Speed => {
                **text = format!("{:.0}", speed_value);
            }
            GaugeKind::Accel => {
                **text = format!("{:.0}", accel_value);
            }
        }
    }
}
