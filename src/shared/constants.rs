//! Shared gameplay and UI constants.

use bevy::prelude::Vec3;

// --- Road ---
pub const ROAD_HALF_WIDTH: f32 = 6.0;
pub const LANE_POSITIONS: [f32; 3] = [-4.0, 0.0, 4.0];
pub const SEGMENT_LENGTH: f32 = 24.0;
pub const SEGMENTS_AHEAD: i32 = 8;
pub const SEGMENTS_BEHIND: i32 = 2;

// --- Car physics ---
pub const CAR_SPEED_MIN: f32 = 0.0;
pub const CAR_SPEED_MAX: f32 = 285.0;
pub const CAR_ACCEL: f32 = 55.0;
pub const CAR_BRAKE: f32 = 90.0;
pub const CAR_STEER: f32 = 22.0;
pub const CAR_HALF_EXTENTS: Vec3 = Vec3::new(1.1, 0.6, 2.2);

// --- Gauge faces ---
pub const GAUGE_SPEED_MIN: f32 = 0.0;
pub const GAUGE_SPEED_MAX: f32 = 285.0;
pub const GAUGE_ACCEL_MIN: f32 = 0.0;
pub const GAUGE_ACCEL_MAX: f32 = 8000.0;

pub const GAUGE_SIZE: f32 = 168.0;
pub const GAUGE_NEEDLE_LEN: f32 = 118.0;
/// Needle sweep: classic dial from ~7:30 to ~4:30 (clockwise).
pub const GAUGE_START_RAD: f32 = 225.0_f32 * (std::f32::consts::PI / 180.0);
pub const GAUGE_SWEEP_RAD: f32 = 270.0_f32 * (std::f32::consts::PI / 180.0);

// --- Environment / day-night ---
/// Full day → night → day cycle length in seconds.
pub const DAY_NIGHT_PERIOD_SECS: f32 = 60.0;
pub const SKY_DOME_RADIUS: f32 = 500.0;
pub const GROUND_SIZE: f32 = 600.0;
