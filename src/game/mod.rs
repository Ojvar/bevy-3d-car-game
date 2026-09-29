//! Root game plugin: resources, startup, and ordered update systems.

mod input;
pub mod scores;
pub mod settings;
mod setup;

use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;

use crate::camera;
use crate::car;
use crate::shared::resources::{
    DayNightCycle, Difficulty, GamePhase, GameState, ObstacleSpawner, PlayerName, RoadTracker,
    Scoreboard,
};
use crate::ui::settings as settings_ui;
use crate::ui::{gauges, hud, name_entry, scoreboard};
use crate::world::{environment, obstacles, road};

/// Orders gameplay systems so driving runs before world / UI updates.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameSet {
    Drive,
    World,
    Ui,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        let playing = in_state(GamePhase::Playing);

        app.init_state::<GamePhase>()
            .init_resource::<GameState>()
            .init_resource::<RoadTracker>()
            .init_resource::<ObstacleSpawner>()
            .init_resource::<DayNightCycle>()
            .init_resource::<PlayerName>()
            .insert_resource(Difficulty::load())
            .insert_resource(Scoreboard::load())
            .configure_sets(
                Update,
                (GameSet::Drive, GameSet::World, GameSet::Ui).chain(),
            )
            .configure_sets(Update, GameSet::Drive.run_if(playing.clone()))
            .add_systems(Startup, setup::setup)
            .add_systems(OnEnter(GamePhase::NameEntry), name_entry::spawn_name_entry)
            .add_systems(OnExit(GamePhase::NameEntry), name_entry::despawn_name_entry)
            .add_systems(OnEnter(GamePhase::Playing), input::reset_run)
            .add_systems(OnEnter(GamePhase::Settings), settings_ui::spawn_settings)
            .add_systems(
                OnExit(GamePhase::Settings),
                (settings_ui::despawn_settings, settings::save_settings),
            )
            .add_systems(
                Update,
                (
                    name_entry::type_player_name,
                    settings_ui::open_settings_on_click,
                )
                    .run_if(in_state(GamePhase::NameEntry)),
            )
            .add_systems(
                Update,
                (settings_ui::settings_keyboard, settings_ui::settings_clicks)
                    .run_if(in_state(GamePhase::Settings)),
            )
            .add_systems(
                Update,
                settings_ui::style_menu_buttons.run_if(not(in_state(GamePhase::Playing))),
            )
            .add_systems(
                Update,
                (
                    car::drive_car,
                    car::toggle_car_lights,
                    car::tag_car_model_lamps,
                )
                    .in_set(GameSet::Drive),
            )
            .add_systems(
                Update,
                (
                    camera::follow_camera,
                    environment::update_day_night,
                    environment::follow_environment.after(camera::follow_camera),
                    road::maintain_infinite_road,
                    obstacles::despawn_passed_obstacles,
                    input::quit_on_escape.run_if(not(in_state(GamePhase::Settings))),
                    (
                        obstacles::spawn_obstacles,
                        obstacles::check_collisions,
                        scores::record_score_on_crash.after(obstacles::check_collisions),
                        input::reset_run.run_if(input_just_pressed(KeyCode::KeyR)),
                        input::change_driver_on_crash,
                    )
                        .run_if(playing),
                )
                    .in_set(GameSet::World),
            )
            .add_systems(
                Update,
                (
                    hud::update_hud,
                    gauges::update_gauges,
                    scoreboard::toggle_scoreboard,
                    scoreboard::refresh_scoreboard,
                )
                    .in_set(GameSet::Ui),
            );
    }
}
