//! ECS components shared across systems.

use bevy::prelude::*;

#[derive(Component)]
pub struct Car {
    pub speed: f32,
    /// Instantaneous acceleration (units/s²). Positive = accel, negative = brake.
    pub acceleration: f32,
}

#[derive(Component)]
pub struct FollowCamera;

#[derive(Component)]
pub struct RoadSegment {
    pub index: i32,
}

#[derive(Component)]
pub struct Obstacle {
    pub half_extents: Vec3,
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum GaugeKind {
    Speed,
    Accel,
}

#[derive(Component)]
pub struct GaugeNeedle;

#[derive(Component)]
pub struct GaugeValueText;

#[derive(Component)]
pub struct HudText;
