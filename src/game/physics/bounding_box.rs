use bevy::math::Vec2;

use super::util::segments_intersect;

/// AABB: Axis-aligned bounding box.
#[derive(Debug, Default, Clone)]
pub struct BoundingBox {
    /// Bottom left point of the box
    pub min: Vec2,
    /// Top right point of the box
    pub max: Vec2,
}

impl BoundingBox {
    /// Construct a bounding box from bottom left and top right corners.
    pub fn from_min_max(min_point: Vec2, max_point: Vec2) -> Self {
        Self {
            min: min_point,
            max: max_point,
        }
    }

    /// Returns whether two bounding boxes collide.
    pub fn collides(&self, other: &BoundingBox) -> bool {
        segments_intersect((self.min.x, self.max.x), (other.min.x, other.max.x))
            && segments_intersect(
                (self.min.y, self.max.y),
                (other.min.y, other.max.y),
            )
    }
}
