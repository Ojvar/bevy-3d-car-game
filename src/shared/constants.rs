//! Shared gameplay and UI constants.

use bevy::prelude::Vec3;

// --- Road ---
pub const ROAD_HALF_WIDTH: f32 = 6.0;
pub const LANE_POSITIONS: [f32; 3] = [-4.0, 0.0, 4.0];
pub const SEGMENT_LENGTH: f32 = 24.0;
pub const SEGMENTS_AHEAD: i32 = 8;
pub const SEGMENTS_BEHIND: i32 = 2;

// --- Car physics (speed values are km/h; convert with KMH_TO_WORLD for movement) ---
pub const CAR_SPEED_MIN: f32 = 40.0;
pub const CAR_SPEED_MAX: f32 = 285.0;
pub const CAR_ACCEL: f32 = 55.0;
pub const CAR_BRAKE: f32 = 90.0;
pub const CAR_STEER: f32 = 22.0;
/// 1 world unit ≈ 1 meter: km/h → m/s.
pub const KMH_TO_WORLD: f32 = 1.0 / 3.6;
/// Matches the glTF car (~1.7 m wide, ~4.7 m long).
pub const CAR_HALF_EXTENTS: Vec3 = Vec3::new(0.85, 0.6, 2.35);

// --- Car model (glTF, relative to the `assets/` folder) ---
/// When this file is missing, the procedural box car is used instead.
pub const CAR_MODEL_PATH: &str = "models/car/scene.gltf";
pub const CAR_MODEL_SCALE: f32 = 1.0;
/// Yaw applied to the model so its front faces +Z (the driving direction).
pub const CAR_MODEL_YAW: f32 = 0.0;
/// Centers the model on the car root and puts its tires on the asphalt
/// (car root sits 0.55 above the road; the model's origin is mid-body).
pub const CAR_MODEL_OFFSET: Vec3 = Vec3::new(-0.09, 0.30, 0.377);
/// Front face of the model in car space, where the headlight beams start.
pub const CAR_MODEL_FRONT_Z: f32 = 2.3;
pub const CAR_MODEL_HEADLIGHT_X: f32 = 0.62;
/// Emissive strength of the model's baked head/tail lamp texture per mode.
pub const CAR_MODEL_LAMP_SHORT: f32 = 2.0;
pub const CAR_MODEL_LAMP_LONG: f32 = 4.0;

// --- Headlights (short = low beam, long = high beam) ---
pub const HEADLIGHT_SHORT_INTENSITY: f32 = 1_800_000.0;
pub const HEADLIGHT_SHORT_RANGE: f32 = 35.0;
pub const HEADLIGHT_SHORT_INNER: f32 = 0.28;
pub const HEADLIGHT_SHORT_OUTER: f32 = 0.55;
pub const HEADLIGHT_SHORT_AIM_Y: f32 = -2.2;
pub const HEADLIGHT_SHORT_AIM_Z: f32 = 18.0;

pub const HEADLIGHT_LONG_INTENSITY: f32 = 4_200_000.0;
pub const HEADLIGHT_LONG_RANGE: f32 = 85.0;
pub const HEADLIGHT_LONG_INNER: f32 = 0.12;
pub const HEADLIGHT_LONG_OUTER: f32 = 0.32;
pub const HEADLIGHT_LONG_AIM_Y: f32 = -0.35;
pub const HEADLIGHT_LONG_AIM_Z: f32 = 48.0;

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
