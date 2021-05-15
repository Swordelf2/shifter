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

use crate::config::physics;
use crate::util::{TransformExt, Vec2Ext};

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
        transform.translate(dynamic_object.vel * delta);
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
    for (entity1, transform1, mut collider1, _dyn_object) in
        dyn_object_query.iter_mut()
    {
        for (entity2, transform2, mut collider2) in stat_object_query.iter_mut()
        {
            if let Some(mut mpv) = collider1.process_collision(&collider2) {
                // Sometimes the mpv might be in the wrong direction
                // Simple (but wrong) fix
                if mpv.dot(
                    (transform2.translation - transform1.translation)
                        .truncate(),
                ) < 0.0
                {
                    mpv = -mpv;
                }
                collider1.add_recent_collision(entity2, mpv);
                collider2.add_recent_collision(entity1, -mpv);
            }
        }
    }

    /**** Bounce ****/
    // (Which entity to bounce, bounce mpv)
    let mut bounces: Vec<(Entity, Vec2)> = Vec::new();

    // Safety: no muts, so no mutable references are retreived
    for (entity, _transform, collider, _dyn_object) in
        unsafe { dyn_object_query.iter_unsafe() }
    {
        for collision in collider.get_recent_collisions() {
            // Safety: no muts again
            // TODO: stat_object_query -> concat(dyn + stat)
            if let Ok(other_collider) = stat_object_query
                .get_component::<Collider>(collision.other_entity)
            {
                if other_collider.solid {
                    // To push self out of other (instead of other out of self), invert mpv
                    bounces.push((entity, -collision.mpv));
                }
            }
        }
    }

    for (entity, mpv) in bounces {
        let (_, mut transform, _, mut dyn_object) =
            dyn_object_query.get_mut(entity).unwrap();
        // Push the object with mpv
        // TODO maybe tranlsate more?
        transform.translation += Vec3::from((mpv, 0.0));

        // Reflect the velocity off of mpv
        dyn_object.vel = physics::BOUNCINESS * dyn_object.vel.reflect(mpv);
    }

    // TODO process collisions between dynamic objects
}
