//! Score / controls HUD text.

use bevy::prelude::*;

use crate::shared::components::{Car, HudText};
use crate::shared::resources::GameState;

pub fn spawn_hud(commands: &mut Commands) {
    commands.spawn((
        Text::new(
            "Bevy Car Rider\nA/D steer  W accel  S brake  L lights  R restart\nScore: 0  Lights: OFF",
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
}

pub fn update_hud(
    state: Res<GameState>,
    car_query: Query<&Car>,
    mut hud: Query<&mut Text, With<HudText>>,
) {
    let Ok(mut text) = hud.single_mut() else {
        return;
    };
    let lights = car_query
        .single()
        .map(|car| car.lights.label())
        .unwrap_or("OFF");

    if state.crashed {
        **text = format!(
            "CRASHED!  Score: {:.0}\nPress R to restart\nA/D steer  W accel  S brake  L lights",
            state.score
        );
    } else {
        **text = format!(
            "Bevy Car Rider\nA/D steer  W accel  S brake  L lights  R restart\nScore: {:.0}  Lights: {}",
            state.score, lights
        );
    }
}
