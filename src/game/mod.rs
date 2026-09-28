//! Root game plugin: resources, startup, and ordered update systems.

mod input;
mod setup;

use bevy::prelude::*;

use crate::camera;
use crate::car;
use crate::shared::resources::{DayNightCycle, GameState, ObstacleSpawner, RoadTracker};
use crate::ui::{gauges, hud};
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
        app.init_resource::<GameState>()
            .init_resource::<RoadTracker>()
            .init_resource::<ObstacleSpawner>()
            .init_resource::<DayNightCycle>()
            .configure_sets(
                Update,
                (GameSet::Drive, GameSet::World, GameSet::Ui).chain(),
            )
            .add_systems(Startup, setup::setup)
            .add_systems(
                Update,
                (car::drive_car, car::toggle_car_lights).in_set(GameSet::Drive),
            )
            .add_systems(
                Update,
                (
                    camera::follow_camera,
                    environment::update_day_night,
                    environment::follow_environment.after(camera::follow_camera),
                    road::maintain_infinite_road,
                    obstacles::spawn_obstacles,
                    obstacles::despawn_passed_obstacles,
                    obstacles::check_collisions,
                    input::restart_on_crash,
                    input::quit_on_escape,
                )
                    .in_set(GameSet::World),
            )
            .add_systems(
                Update,
                (hud::update_hud, gauges::update_gauges).in_set(GameSet::Ui),
            );
    }
}
