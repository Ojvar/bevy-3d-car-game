//! Top-left driver / score panel and controls hint.

use bevy::prelude::*;

use crate::car::score_multiplier;
use crate::shared::components::{Car, HudText};
use crate::shared::resources::{GameState, PlayerName, Scoreboard};

pub fn spawn_hud(commands: &mut Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(16.0),
            left: Val::Px(16.0),
            padding: UiRect::axes(Val::Px(14.0), Val::Px(10.0)),
            border_radius: BorderRadius::all(Val::Px(8.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.04, 0.05, 0.08, 0.6)),
        children![(
            Text::new("SCORE 0"),
            TextFont {
                font_size: FontSize::Px(22.0),
                ..default()
            },
            TextColor(Color::srgb(0.95, 0.95, 0.97)),
            HudText,
        )],
    ));

    commands.spawn((
        Text::new("A/D steer   W accel   S brake   L lights   R restart   Esc quit"),
        TextFont {
            font_size: FontSize::Px(14.0),
            ..default()
        },
        TextColor(Color::srgb(0.95, 0.95, 0.97)),
        TextShadow::default(),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(16.0),
            left: Val::Px(16.0),
            ..default()
        },
    ));
}

pub fn update_hud(
    state: Res<GameState>,
    player: Res<PlayerName>,
    board: Res<Scoreboard>,
    car_query: Query<&Car>,
    mut hud: Query<&mut Text, With<HudText>>,
) {
    let Ok(mut text) = hud.single_mut() else {
        return;
    };
    let (lights, bonus) = car_query
        .single()
        .map(|car| (car.lights.label(), score_multiplier(car.speed)))
        .unwrap_or(("OFF", 1.0));

    let driver = if player.0.is_empty() { "—" } else { &player.0 };
    let best = board.best().unwrap_or(0).max(state.score.round() as u32);

    **text = format!(
        "DRIVER  {driver}\nSCORE   {:.0}\nBEST    {best}\nBonus x{bonus:.1}   Lights {lights}",
        state.score
    );
}
