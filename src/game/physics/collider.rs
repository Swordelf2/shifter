use bevy::ecs::entity::Entity;
use bevy::math::Vec2;
use bevy::transform::components::Transform;

use super::shape::{Shape, ShiftedShape};
use super::util::{update_max_point, update_min_point};
use super::BoundingBox;

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

    #[inline]
    pub(super) fn add_recent_collision(&mut self, other_entity: Entity) {
        self.recent_collisions.push(Collision {
            entity: other_entity,
        });
    }

    /// Update the collider's global shapes and bounding box.
    /// Also clears `recent_collisions`
    pub(super) fn update(&mut self, transform: &Transform) {
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
            update_min_point(&mut min_point, shape_min_point);
            update_max_point(&mut max_point, shape_max_point);
        }

        self.bounding_box = BoundingBox::from_min_max(min_point, max_point);
        self.recent_collisions.clear();
    }

    /// Returns whether two colliders are colliding.
    pub(super) fn process_collision(&self, other: &Collider) -> bool {
        // TODO impl real stuff
        self.bounding_box.collides(&other.bounding_box)
    }
}