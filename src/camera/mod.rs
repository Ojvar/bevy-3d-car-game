//! Camera follow behavior.

use bevy::prelude::*;

use crate::shared::components::{Car, FollowCamera};

pub fn follow_camera(
    car_query: Query<&Transform, (With<Car>, Without<FollowCamera>)>,
    mut cam_query: Query<&mut Transform, With<FollowCamera>>,
) {
    let Ok(car) = car_query.single() else {
        return;
    };
    let Ok(mut cam) = cam_query.single_mut() else {
        return;
    };

    let target = Vec3::new(
        car.translation.x * 0.35,
        car.translation.y + 7.5,
        car.translation.z - 14.0,
    );
    cam.translation = cam.translation.lerp(target, 0.12);
    cam.look_at(car.translation + Vec3::new(0.0, 1.0, 10.0), Vec3::Y);
}
