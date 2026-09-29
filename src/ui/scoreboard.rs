//! End-of-run leaderboard shown after a crash.

use bevy::prelude::*;

use crate::shared::constants::SCOREBOARD_SHOWN;
use crate::shared::resources::{GamePhase, GameState, Scoreboard};

#[derive(Component)]
pub struct ScoreboardPanel;

#[derive(Component)]
pub struct ScoreboardRows;

#[derive(Component)]
pub struct ScoreboardHeadline;

const GOLD: Color = Color::srgb(0.95, 0.75, 0.2);
const ROW_TEXT: Color = Color::srgb(0.85, 0.86, 0.9);
const ROW_HIGHLIGHT_BG: Color = Color::srgba(0.95, 0.75, 0.2, 0.25);

pub fn spawn_scoreboard(commands: &mut Commands) {
    commands.spawn((
        ScoreboardPanel,
        Visibility::Hidden,
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
                row_gap: Val::Px(10.0),
                padding: UiRect::axes(Val::Px(36.0), Val::Px(24.0)),
                border: UiRect::all(Val::Px(2.0)),
                border_radius: BorderRadius::all(Val::Px(12.0)),
                width: Val::Px(440.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.04, 0.05, 0.08, 0.88)),
            BorderColor::all(GOLD),
            children![
                (
                    Text::new("CRASHED!"),
                    TextFont {
                        font_size: FontSize::Px(32.0),
                        ..default()
                    },
                    TextColor(Color::srgb(0.95, 0.3, 0.25)),
                ),
                (
                    Text::new(""),
                    TextFont {
                        font_size: FontSize::Px(20.0),
                        ..default()
                    },
                    TextColor(GOLD),
                    ScoreboardHeadline,
                ),
                (
                    Text::new("TOP SCORES"),
                    TextFont {
                        font_size: FontSize::Px(16.0),
                        ..default()
                    },
                    TextColor(Color::srgb(0.6, 0.62, 0.68)),
                ),
                (
                    ScoreboardRows,
                    Node {
                        flex_direction: FlexDirection::Column,
                        width: Val::Percent(100.0),
                        row_gap: Val::Px(2.0),
                        ..default()
                    },
                ),
                (
                    Text::new("R  race again     N  change driver     Esc  quit"),
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

pub fn toggle_scoreboard(
    state: Res<GameState>,
    phase: Res<State<GamePhase>>,
    mut panel: Query<&mut Visibility, With<ScoreboardPanel>>,
) {
    let Ok(mut visibility) = panel.single_mut() else {
        return;
    };
    let shown = state.crashed && *phase.get() == GamePhase::Playing;
    visibility.set_if_neq(if shown {
        Visibility::Inherited
    } else {
        Visibility::Hidden
    });
}

pub fn refresh_scoreboard(
    mut commands: Commands,
    board: Res<Scoreboard>,
    state: Res<GameState>,
    rows: Query<Entity, With<ScoreboardRows>>,
    mut headline: Query<&mut Text, With<ScoreboardHeadline>>,
) {
    if !board.is_changed() {
        return;
    }

    if let Ok(mut text) = headline.single_mut() {
        **text = match board.last_run {
            Some(0) => format!("NEW HIGH SCORE!  {:.0}", state.score),
            Some(rank) => format!("Score {:.0}  —  rank #{}", state.score, rank + 1),
            None => format!("Score {:.0}", state.score),
        };
    }

    let Ok(rows) = rows.single() else {
        return;
    };
    commands.entity(rows).despawn_children();

    if board.entries.is_empty() {
        let empty = commands
            .spawn((
                Text::new("No runs yet"),
                TextFont {
                    font_size: FontSize::Px(16.0),
                    ..default()
                },
                TextColor(ROW_TEXT),
            ))
            .id();
        commands.entity(rows).add_child(empty);
        return;
    }

    let mut shown: Vec<usize> = (0..board.entries.len().min(SCOREBOARD_SHOWN)).collect();
    // Always show the latest run, even when it's below the visible top list.
    if let Some(last) = board.last_run.filter(|&i| i >= SCOREBOARD_SHOWN) {
        shown.push(last);
    }

    for index in shown {
        let entry = &board.entries[index];
        let is_last_run = board.last_run == Some(index);
        let color = if is_last_run { GOLD } else { ROW_TEXT };
        let row = commands
            .spawn((
                Node {
                    justify_content: JustifyContent::SpaceBetween,
                    padding: UiRect::axes(Val::Px(10.0), Val::Px(3.0)),
                    border_radius: BorderRadius::all(Val::Px(4.0)),
                    ..default()
                },
                BackgroundColor(if is_last_run {
                    ROW_HIGHLIGHT_BG
                } else {
                    Color::NONE
                }),
                children![
                    (
                        Text::new(format!("{:>2}.  {}", index + 1, entry.name)),
                        TextFont {
                            font_size: FontSize::Px(18.0),
                            ..default()
                        },
                        TextColor(color),
                    ),
                    (
                        Text::new(entry.score.to_string()),
                        TextFont {
                            font_size: FontSize::Px(18.0),
                            ..default()
                        },
                        TextColor(color),
                    ),
                ],
            ))
            .id();
        commands.entity(rows).add_child(row);
    }
}
