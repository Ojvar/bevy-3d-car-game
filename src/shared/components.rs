//! ECS components shared across systems.

use bevy::prelude::*;

#[derive(Component)]
pub struct Car {
    pub speed: f32,
    /// Instantaneous acceleration (km/h per second). Positive = accel, negative = brake.
    pub acceleration: f32,
    pub lights: HeadlightMode,
}

/// Off → short (low beam) → long (high beam).
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum HeadlightMode {
    #[default]
    Off,
    Short,
    Long,
}

impl HeadlightMode {
    pub fn next(self) -> Self {
        match self {
            Self::Off => Self::Short,
            Self::Short => Self::Long,
            Self::Long => Self::Off,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Off => "OFF",
            Self::Short => "SHORT",
            Self::Long => "LONG",
        }
    }
}

/// Spot light beam child of the car.
#[derive(Component)]
pub struct CarHeadlight;

/// Glowing headlight lens mesh on the bumper.
#[derive(Component)]
pub struct CarHeadlightLens;

/// Root of the loaded glTF car scene.
#[derive(Component)]
pub struct CarModel;

/// Mesh inside the glTF car whose material carries the baked head/tail lamp glow.
#[derive(Component)]
pub struct CarModelLamp;

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
