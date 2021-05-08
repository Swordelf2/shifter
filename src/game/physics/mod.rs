//! Every entity that can move in physics environment has [DynamicObject] component.
//! Every entity that can collide with any other object has [Collider] component.
//! Every [Collider] entity that has [DynamicObject] component is considered a
//! "dynamic collider object", otherwise it's a "static collider object".
//!
pub use bounding_box::BoundingBox;
pub use collider::{Collider, Collision};
pub use dynamic_object::DynamicObject;

pub mod shape;
pub mod util;

mod bounding_box;
mod collider;
mod dynamic_object;

use bevy::prelude::*;

/// Main physics system, moves all dynamic objects and processes collisions
pub fn update(
    time: Res<Time>,
    mut dyn_object_query: Query<(
        Entity,
        &mut Transform,
        &mut Collider,
        &mut DynamicObject,
    )>,
    mut stat_object_query: Query<
        (Entity, &Transform, &mut Collider),
        Without<DynamicObject>,
    >,
) {
    /*** Movement ***/
    for (_entity, mut transform, _collider, mut dynamic_object) in
        dyn_object_query.iter_mut()
    {
        let delta = time.delta_seconds();

        let dynamic_object = &mut *dynamic_object;

        // Apply friction
        let total_accel = dynamic_object.accel
            - dynamic_object.vel * dynamic_object.friction_coeff;
        // Apply the aceleration
        dynamic_object.vel += total_accel * delta;
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

    /**** Collision ****/
    // Update colliders
    for (_entity, transform, mut collider, _dyn_object) in
        dyn_object_query.iter_mut()
    {
        collider.update(&transform);
    }
    for (_entity, transform, mut collider) in stat_object_query.iter_mut() {
        collider.update(&transform);
    }

    // Process collisions between all pairs of a dynamic and a static object
    for (entity1, _transform1, mut collider1, _dyn_object) in
        dyn_object_query.iter_mut()
    {
        for (entity2, _transform2, mut collider2) in
            stat_object_query.iter_mut()
        {
            if let Some(mpv) = collider1.process_collision(&collider2) {
                collider1.add_recent_collision(entity2, mpv);
                collider2.add_recent_collision(entity1, -mpv);
            }
        }
    }
}
