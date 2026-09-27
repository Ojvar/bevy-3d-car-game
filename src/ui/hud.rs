//! Score / controls HUD text.

use bevy::prelude::*;

use crate::shared::components::HudText;
use crate::shared::resources::GameState;

pub fn spawn_hud(commands: &mut Commands) {
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
}

pub fn update_hud(state: Res<GameState>, mut hud: Query<&mut Text, With<HudText>>) {
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
