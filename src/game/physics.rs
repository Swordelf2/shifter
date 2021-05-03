//! Every entity that can move in physics environment has [DynamicObject] component.
//! Every entity that can collide with any other object has [Collider] component.
//! Every [Collider] entity that has [DynamicObject] component is considered a
//! "dynamic collider object", otherwise it's a "static collider object".
//!
use bevy::prelude::*;
use bevy::sprite::collide_aabb;

use crate::config::physics;
use crate::util::shape;
use crate::util::shape::{Shape, ShiftedShape};

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
    /// Returns a dynamic object with global maximum velocity.
    pub fn new() -> Self {
        Self {
            accel: Default::default(),
            vel: Default::default(),
            max_vel_squared: physics::GLOBAL_MAX_VEL * physics::GLOBAL_MAX_VEL,
        }
    }

    pub fn from_max_vel(max_vel: f32) -> Self {
        assert!(
            max_vel <= physics::GLOBAL_MAX_VEL,
            "`max_vel` is greater than `GLOBAL_MAX_VEL`"
        );
        Self {
            max_vel_squared: max_vel * max_vel,
            ..Self::new()
        }
    }
}

/// Component, indicating that this entity can collide with other colliders.
#[derive(Debug, Default)]
#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
pub struct Collider {
    /// Shapes that comprise the collider.
    shapes: Vec<ShiftedShape>,
    /// Solid colliders are bounced off of, nonsolid can be passed through.
    solid: bool,
    /// Collision instances that happened within the last frame.
    /// This is cleared and set in `physics::update()` system each frame.
    recent_collisions: Vec<Collision>,
    /// AABB, used for collision optimization
    bounding_box: BoundingBox,
}

/// AABB: Axis-aligned bounding box.
#[derive(Debug, Default)]
#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
pub struct BoundingBox {
    /// Position of the middle of the bounding box.
    /// Z coordinate is always 0.
    pub position: Vec3,
    pub size: Vec2,
}

impl BoundingBox {
    /// Construct a bounding box from bottom left and top right corners.
    pub fn from_min_max(min_point: Vec2, max_point: Vec2) -> Self {
        Self {
            position: Vec3::new(
                (max_point.x + min_point.x) * 0.5,
                (max_point.y + min_point.y) * 0.5,
                0.0,
            ),
            size: Vec2::new(
                max_point.x - min_point.x,
                max_point.y - min_point.y,
            ),
        }
    }

    /// Returns whether two bounding boxes collide.
    pub fn collides(&self, other: &BoundingBox) -> bool {
        collide_aabb::collide(
            self.position,
            self.size,
            other.position,
            other.size,
        )
        .is_some()
    }
}

/// Collision instance
#[derive(Debug)]
#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
pub struct Collision {
    pub entity: Entity,
}

// TODO probalby should remove this Default, it's only here for the inspector
impl Default for Collision {
    fn default() -> Self {
        Self {
            entity: Entity::new(0),
        }
    }
}

impl Collider {
    /// Create a collider with given shapes. `shapes` must not be empty
    pub fn nonsolid_from_shapes(shapes: Vec<Shape>) -> Self {
        assert!(!shapes.is_empty());
        Self {
            shapes: shapes
                .into_iter()
                .map(ShiftedShape::from_original_shape)
                .collect(),
            solid: false,
            recent_collisions: Vec::new(),
            bounding_box: BoundingBox::default(),
        }
    }

    pub fn solid_from_shapes(shapes: Vec<Shape>) -> Self {
        Self {
            solid: true,
            ..Self::nonsolid_from_shapes(shapes)
        }
    }

    /// Return all collision instances that happened during this frame
    #[inline]
    pub fn get_recent_collisions(&self) -> &[Collision] {
        &self.recent_collisions
    }

    /// Update the collider's global shapes and bounding box.
    /// Also clears `recent_collisions`
    fn update(&mut self, transform: &Transform) {
        assert!(!self.shapes.is_empty());
        // Bottom left of the bounding box
        let mut min_point: Vec2 = Vec2::new(f32::INFINITY, f32::INFINITY);
        // Top right of the bounding box
        let mut max_point: Vec2 =
            Vec2::new(f32::NEG_INFINITY, f32::NEG_INFINITY);
        // Iterate and update all shapes according to current `transform`,
        // while also capturing min and max points
        for shape in &mut self.shapes {
            let (shape_min_point, shape_max_point) = shape.update(&transform);
            shape::update_min_point(&mut min_point, shape_min_point);
            shape::update_max_point(&mut max_point, shape_max_point);
        }

        self.bounding_box = BoundingBox::from_min_max(min_point, max_point);
        self.recent_collisions.clear();
    }

    /// Returns whether two colliders are colliding.
    fn process_collision(&self, other: &Collider) -> bool {
        // TODO impl real stuff
        self.bounding_box.collides(&other.bounding_box)
    }
}

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
            if collider1.process_collision(&collider2) {
                collider1
                    .recent_collisions
                    .push(Collision { entity: entity2 });
                collider2
                    .recent_collisions
                    .push(Collision { entity: entity1 });
            }
        }
    }
}
