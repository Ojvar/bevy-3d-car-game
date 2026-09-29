//! Settings menu opened from the name entry screen: pick the game level.

use bevy::prelude::*;

use crate::shared::resources::{Difficulty, GamePhase};

#[derive(Component)]
pub struct SettingsPanel;

#[derive(Component)]
pub struct DifficultyButton(pub Difficulty);

#[derive(Component)]
pub struct SettingsBackButton;

#[derive(Component)]
pub struct DifficultyDescription;

/// Name entry screen button that opens this menu.
#[derive(Component)]
pub struct OpenSettingsButton;

const GOLD: Color = Color::srgb(0.95, 0.75, 0.2);
const MUTED: Color = Color::srgb(0.6, 0.62, 0.68);
const BUTTON_BG: Color = Color::srgb(0.1, 0.11, 0.14);
const BUTTON_HOVER_BG: Color = Color::srgb(0.18, 0.19, 0.24);
const BUTTON_SELECTED_BG: Color = Color::srgba(0.95, 0.75, 0.2, 0.28);
const BUTTON_BORDER: Color = Color::srgb(0.4, 0.42, 0.48);

pub fn menu_button(label: impl Into<String>, font_size: f32) -> impl Bundle {
    (
        Button,
        Node {
            padding: UiRect::axes(Val::Px(18.0), Val::Px(8.0)),
            min_width: Val::Px(110.0),
            justify_content: JustifyContent::Center,
            border: UiRect::all(Val::Px(2.0)),
            border_radius: BorderRadius::all(Val::Px(6.0)),
            ..default()
        },
        BackgroundColor(BUTTON_BG),
        BorderColor::all(BUTTON_BORDER),
        children![(
            Text::new(label),
            TextFont {
                font_size: FontSize::Px(font_size),
                ..default()
            },
            TextColor(Color::WHITE),
        )],
    )
}

pub fn spawn_settings(mut commands: Commands, difficulty: Res<Difficulty>) {
    commands.spawn((
        SettingsPanel,
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
                row_gap: Val::Px(16.0),
                padding: UiRect::axes(Val::Px(40.0), Val::Px(28.0)),
                border: UiRect::all(Val::Px(2.0)),
                border_radius: BorderRadius::all(Val::Px(12.0)),
                min_width: Val::Px(460.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.04, 0.05, 0.08, 0.88)),
            BorderColor::all(GOLD),
            children![
                (
                    Text::new("SETTINGS"),
                    TextFont {
                        font_size: FontSize::Px(34.0),
                        ..default()
                    },
                    TextColor(GOLD),
                ),
                (
                    Text::new("Game level"),
                    TextFont {
                        font_size: FontSize::Px(20.0),
                        ..default()
                    },
                    TextColor(Color::srgb(0.85, 0.86, 0.9)),
                ),
                (
                    Node {
                        column_gap: Val::Px(12.0),
                        ..default()
                    },
                    children![
                        (
                            DifficultyButton(Difficulty::Easy),
                            menu_button(Difficulty::Easy.label(), 22.0)
                        ),
                        (
                            DifficultyButton(Difficulty::Medium),
                            menu_button(Difficulty::Medium.label(), 22.0)
                        ),
                        (
                            DifficultyButton(Difficulty::Hard),
                            menu_button(Difficulty::Hard.label(), 22.0)
                        ),
                    ],
                ),
                (
                    Text::new(difficulty.description()),
                    TextFont {
                        font_size: FontSize::Px(16.0),
                        ..default()
                    },
                    TextColor(Color::srgb(0.85, 0.86, 0.9)),
                    DifficultyDescription,
                ),
                (SettingsBackButton, menu_button("BACK", 18.0)),
                (
                    Text::new("Left/Right  change     Enter / Esc  back"),
                    TextFont {
                        font_size: FontSize::Px(14.0),
                        ..default()
                    },
                    TextColor(MUTED),
                ),
            ],
        )],
    ));
}

pub fn despawn_settings(mut commands: Commands, panels: Query<Entity, With<SettingsPanel>>) {
    for entity in &panels {
        commands.entity(entity).despawn();
    }
}

pub fn settings_keyboard(
    keys: Res<ButtonInput<KeyCode>>,
    mut difficulty: ResMut<Difficulty>,
    mut next_phase: ResMut<NextState<GamePhase>>,
) {
    if keys.any_just_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
        let prev = difficulty.prev();
        difficulty.set_if_neq(prev);
    }
    if keys.any_just_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        let next = difficulty.next();
        difficulty.set_if_neq(next);
    }
    if keys.any_just_pressed([
        KeyCode::Enter,
        KeyCode::NumpadEnter,
        KeyCode::Escape,
        KeyCode::Tab,
    ]) {
        next_phase.set(GamePhase::NameEntry);
    }
}

pub fn settings_clicks(
    level_buttons: Query<(&Interaction, &DifficultyButton), Changed<Interaction>>,
    back_buttons: Query<&Interaction, (Changed<Interaction>, With<SettingsBackButton>)>,
    mut difficulty: ResMut<Difficulty>,
    mut next_phase: ResMut<NextState<GamePhase>>,
) {
    for (interaction, button) in &level_buttons {
        if *interaction == Interaction::Pressed {
            difficulty.set_if_neq(button.0);
        }
    }
    if back_buttons.iter().any(|i| *i == Interaction::Pressed) {
        next_phase.set(GamePhase::NameEntry);
    }
}

pub fn open_settings_on_click(
    buttons: Query<&Interaction, (Changed<Interaction>, With<OpenSettingsButton>)>,
    mut next_phase: ResMut<NextState<GamePhase>>,
) {
    if buttons.iter().any(|i| *i == Interaction::Pressed) {
        next_phase.set(GamePhase::Settings);
    }
}

/// Highlights the selected level and hovered buttons; runs in every menu phase.
pub fn style_menu_buttons(
    difficulty: Res<Difficulty>,
    mut buttons: Query<
        (
            &Interaction,
            Option<&DifficultyButton>,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        With<Button>,
    >,
    mut description: Query<&mut Text, With<DifficultyDescription>>,
) {
    for (interaction, level, mut background, mut border) in &mut buttons {
        let selected = level.is_some_and(|level| level.0 == *difficulty);
        let bg = if selected {
            BUTTON_SELECTED_BG
        } else if *interaction == Interaction::None {
            BUTTON_BG
        } else {
            BUTTON_HOVER_BG
        };
        let edge = if selected || *interaction != Interaction::None {
            GOLD
        } else {
            BUTTON_BORDER
        };
        background.set_if_neq(BackgroundColor(bg));
        border.set_if_neq(BorderColor::all(edge));
    }

    if difficulty.is_changed()
        && let Ok(mut text) = description.single_mut()
    {
        **text = difficulty.description().to_string();
    }
}
