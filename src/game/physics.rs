use bevy::prelude::*;

use crate::config::physics;

/// Component, indicating that this entity can move and collide with colliders
#[derive(Debug)]
pub struct DynamicObject {
    /// Aceleration
    pub accel: Vec2,

    /// Velocity
    vel: Vec2,

    /// The square of the speed that this object can not exceed
    max_vel_squared: f32,
}

impl DynamicObject {
    /// Create a new Dynamic Object with zero acceleration and velocity and
    /// with given `max_vel`. If the latter is `None` then
    /// `max_vel = config::physics::GLOBAL_MAX_SPEED`
    pub fn new(max_vel: Option<f32>) -> Self {
        let sqr = |val| val * val;
        Self {
            accel: Default::default(),
            vel: Default::default(),
            max_vel_squared: sqr(max_vel.unwrap_or(physics::GLOBAL_MAX_SPEED)),
        }
    }
}

/// Main physics system, moves all dynamic
pub fn movement(
    time: Res<Time>,
    mut dyn_object_query: Query<(&mut Transform, &mut DynamicObject)>,
) {
    for (mut transform, mut dynamic_object) in dyn_object_query.iter_mut() {
        dbg!(&dynamic_object);
        let delta = time.delta_seconds();

        let dynamic_object = &mut *dynamic_object;

        // Apply the aceleration
        dynamic_object.vel += dynamic_object.accel * delta;
        // Clamp the velocity to `max_vel`
        if dynamic_object.vel.length_squared() > dynamic_object.max_vel_squared
        {
            dynamic_object.vel *= f32::sqrt(
                dynamic_object.max_vel_squared
                    / dynamic_object.vel.length_squared(),
            );
        }
        // Apply the velocity
        transform.translation += Vec3::from((dynamic_object.vel, 0.0)) * delta;
    }
}
