//! Driver name prompt shown before each race.

use bevy::input::ButtonState;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::prelude::*;

use crate::shared::constants::PLAYER_NAME_MAX_LEN;
use crate::shared::resources::{Difficulty, GamePhase, PlayerName};
use crate::ui::settings::{OpenSettingsButton, menu_button};

#[derive(Component)]
pub struct NameEntryPanel;

#[derive(Component)]
pub struct NameInputText;

pub fn spawn_name_entry(
    mut commands: Commands,
    player: Res<PlayerName>,
    difficulty: Res<Difficulty>,
) {
    commands.spawn((
        NameEntryPanel,
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        children![(
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(14.0),
                padding: UiRect::axes(Val::Px(40.0), Val::Px(28.0)),
                border: UiRect::all(Val::Px(2.0)),
                border_radius: BorderRadius::all(Val::Px(12.0)),
                min_width: Val::Px(420.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.04, 0.05, 0.08, 0.85)),
            BorderColor::all(Color::srgb(0.95, 0.75, 0.2)),
            children![
                (
                    Text::new("BEVY CAR RIDER"),
                    TextFont {
                        font_size: FontSize::Px(34.0),
                        ..default()
                    },
                    TextColor(Color::srgb(0.95, 0.75, 0.2)),
                ),
                (
                    Text::new("Enter your name"),
                    TextFont {
                        font_size: FontSize::Px(20.0),
                        ..default()
                    },
                    TextColor(Color::srgb(0.85, 0.86, 0.9)),
                ),
                (
                    Node {
                        padding: UiRect::axes(Val::Px(16.0), Val::Px(8.0)),
                        min_width: Val::Px(320.0),
                        justify_content: JustifyContent::Center,
                        border: UiRect::all(Val::Px(1.0)),
                        border_radius: BorderRadius::all(Val::Px(6.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.1, 0.11, 0.14)),
                    BorderColor::all(Color::srgb(0.4, 0.42, 0.48)),
                    children![(
                        Text::new(name_with_cursor(&player.0)),
                        TextFont {
                            font_size: FontSize::Px(28.0),
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        NameInputText,
                    )],
                ),
                (
                    Node {
                        column_gap: Val::Px(12.0),
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    children![
                        (
                            Text::new(format!("Level  {}", difficulty.label())),
                            TextFont {
                                font_size: FontSize::Px(18.0),
                                ..default()
                            },
                            TextColor(Color::srgb(0.95, 0.75, 0.2)),
                        ),
                        (OpenSettingsButton, menu_button("SETTINGS", 16.0)),
                    ],
                ),
                (
                    Text::new("Enter  start     Tab  settings     Backspace  delete     Esc  quit"),
                    TextFont {
                        font_size: FontSize::Px(14.0),
                        ..default()
                    },
                    TextColor(Color::srgb(0.6, 0.62, 0.68)),
                ),
            ],
        )],
    ));
}

pub fn despawn_name_entry(mut commands: Commands, panels: Query<Entity, With<NameEntryPanel>>) {
    for entity in &panels {
        commands.entity(entity).despawn();
    }
}

fn name_with_cursor(name: &str) -> String {
    format!("{name}_")
}

pub fn type_player_name(
    mut keyboard: MessageReader<KeyboardInput>,
    phase: Res<State<GamePhase>>,
    mut player: ResMut<PlayerName>,
    mut next_phase: ResMut<NextState<GamePhase>>,
    mut input_text: Query<&mut Text, With<NameInputText>>,
) {
    // Drop the key that opened this screen (e.g. N) so it isn't typed into the name.
    if phase.is_changed() {
        keyboard.clear();
        return;
    }

    let mut edited = false;
    for event in keyboard.read() {
        if event.state != ButtonState::Pressed {
            continue;
        }
        match &event.logical_key {
            Key::Enter => {
                let trimmed = player.0.trim().to_string();
                if !trimmed.is_empty() {
                    player.0 = trimmed;
                    next_phase.set(GamePhase::Playing);
                }
            }
            Key::Tab => {
                next_phase.set(GamePhase::Settings);
            }
            Key::Backspace => {
                edited |= player.0.pop().is_some();
            }
            _ => {
                let Some(text) = &event.text else {
                    continue;
                };
                for c in text.chars().filter(|c| !c.is_control()) {
                    if player.0.chars().count() >= PLAYER_NAME_MAX_LEN {
                        break;
                    }
                    player.0.push(c);
                    edited = true;
                }
            }
        }
    }

    if edited && let Ok(mut text) = input_text.single_mut() {
        **text = name_with_cursor(&player.0);
    }
}
