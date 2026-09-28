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

/// Primary sun directional light for the day/night cycle.
#[derive(Component)]
pub struct Sun;

/// Dim moonlight used during night.
#[derive(Component)]
pub struct Moon;

/// Large sky dome that follows the camera.
#[derive(Component)]
pub struct SkyDome;

/// Rolling ground plane that follows the car.
#[derive(Component)]
pub struct GroundFollow;

/// Distant hills that scroll with the car along the road.
#[derive(Component)]
pub struct Hill {
    pub lateral_x: f32,
    pub z_offset: f32,
}
