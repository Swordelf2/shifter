use bevy::prelude::*;

use crate::config::physics;
use crate::util::shape::{CircleShape, PolyShape, Shape};

/// Component, indicating that this entity can move and collide with colliders
#[derive(Debug)]
#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
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

/// Component, indicating that this entity can collide with other colliders
#[derive(Debug, Default)]
#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
pub struct Collider {
    /// Non-transformed ('static')
    shapes: Vec<Shape>,
    /// Entities this object has collided with in this frame
    recent_collisions: Vec<Collision>,
}

/// Collision instance
#[derive(Debug, Default)]
#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
pub struct Collision {
    // TODO
}

impl Collider {
    pub fn from_shapes(shapes: Vec<Shape>) -> Self {
        Self {
            shapes,
            recent_collisions: Vec::new(),
        }
    }
}

/// Main physics system, moves all dynamic objects and processes collisions
pub fn update(
    time: Res<Time>,
    mut dyn_object_query: Query<(&mut Transform, &mut DynamicObject)>,
) {
    for (mut transform, mut dynamic_object) in dyn_object_query.iter_mut() {
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

        // TODO collision
    }
}
