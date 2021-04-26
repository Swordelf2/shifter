use bevy::ecs::system::QuerySingleError;
use bevy::math::Mat2;
use bevy::prelude::*;

use crate::config::{keybinds, PLAYER_ACCEL, ROTATION_SPEED};
use crate::util::QuatExt;

use super::physics::DynamicObject;

/// Marker component for the player entity
pub struct Player;

/// Change player's accel in response to player input
pub fn input(
    input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut DynamicObject, &Transform), With<Player>>,
) {
    // TODO: add logging here
    let (mut dynamic_object, transform) = match player_query.single_mut() {
        Ok(dynamic_object) => dynamic_object,
        Err(QuerySingleError::NoEntities(_)) => return,
        Err(QuerySingleError::MultipleEntities(e)) => {
            panic!("Multiple player entities: {}", e)
        }
    };

    let mut accel = Vec2::default();
    if input.pressed(keybinds::movement::LEFT) {
        accel += Vec2::new(-1.0, 0.0);
    }
    if input.pressed(keybinds::movement::RIGHT) {
        accel += Vec2::new(1.0, 0.0);
    }
    if input.pressed(keybinds::movement::UP) {
        accel += Vec2::new(0.0, 1.0);
    }
    if input.pressed(keybinds::movement::DOWN) {
        accel += Vec2::new(0.0, -1.0);
    }
    // Normalize
    let accel = accel.normalize_or_zero();
    // Rotate the vector
    // The acceleration should be relative to the player's rotation
    let accel =
        Mat2::from_angle(QuatExt::to_angle(&transform.rotation)) * accel;
    // Multiply by the accel value and assign
    dynamic_object.accel = accel * PLAYER_ACCEL;
}

/// Move player in response to player actions
pub fn rotation(
    time: Res<Time>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    // TODO add logging here
    let mut transform = match player_query.single_mut() {
        Ok(transform) => transform,
        Err(QuerySingleError::NoEntities(_)) => return,
        Err(QuerySingleError::MultipleEntities(e)) => {
            panic!("Multiple player entities: {}", e);
        }
    };

    QuatExt::rotate(
        &mut transform.rotation,
        time.delta_seconds() * ROTATION_SPEED,
    );
}
