use bevy::ecs::system::QuerySingleError;
use bevy::prelude::*;

use core::f32::consts::PI;

use crate::config::{keybinds, MOVE_SPEED, ROTATION_SPEED};

/// Marker component for the player entity
pub struct Player;

// TODO how to implement PlayerBundle (or smth like that, maybe player::spawn()) correctly?

/// Move player in response to player actions
pub fn player_control(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut transform = match query.single_mut() {
        Ok(transform) => transform,
        Err(QuerySingleError::NoEntities(_)) => {
            return;
        }
        Err(QuerySingleError::MultipleEntities(e)) => {
            panic!("Multiple player entities: {}", e);
        }
    };

    let angle = transform.rotation.to_axis_angle().1
        + time.delta_seconds() * ROTATION_SPEED;
    let angle = if angle >= 2.0 * PI {
        angle - 2.0 * PI
    } else {
        angle
    };

    transform.rotation = Quat::from_rotation_z(angle);

    if input.pressed(keybinds::movement::LEFT) {
        transform.translation += Vec3::new(
            -time.delta_seconds() * MOVE_SPEED * angle.cos(),
            -time.delta_seconds() * MOVE_SPEED * angle.sin(),
            0.0,
        );
    }
    if input.pressed(keybinds::movement::RIGHT) {
        transform.translation += Vec3::new(
            time.delta_seconds() * MOVE_SPEED * angle.cos(),
            time.delta_seconds() * MOVE_SPEED * angle.sin(),
            0.0,
        );
    }
    if input.pressed(keybinds::movement::UP) {
        transform.translation += Vec3::new(
            -time.delta_seconds() * MOVE_SPEED * angle.sin(),
            time.delta_seconds() * MOVE_SPEED * angle.cos(),
            0.0,
        );
    }
    if input.pressed(keybinds::movement::DOWN) {
        transform.translation += Vec3::new(
            time.delta_seconds() * MOVE_SPEED * angle.sin(),
            -time.delta_seconds() * MOVE_SPEED * angle.cos(),
            0.0,
        );
    }
}
