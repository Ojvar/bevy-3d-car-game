//! Endless road car rider — drive a procedural car on an infinite road
//! with dynamically spawned obstacles.
//!
//! Controls:
//!   A / Left  — steer left
//!   D / Right — steer right
//!   W / Up    — accelerate
//!   S / Down  — brake
//!   L         — toggle headlights
//!   R         — restart after crash
//!   Esc       — quit

mod camera;
mod car;
mod shared;
mod game;
mod ui;
mod world;

use bevy::prelude::*;
use game::GamePlugin;

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
        .add_plugins(GamePlugin)
        .run();
}
