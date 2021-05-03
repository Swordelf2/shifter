use bevy::{math::Vec2, transform::components::Transform};

use super::QuatExt;

/// Utility methods for 2D `Transform`s
/// TODO: nothing here is used yet, maybe remove
pub trait TransformExt {
    /// Returns forward direction
    fn forward(&self) -> Vec2;
    /// Returns left direction
    fn left(&self) -> Vec2;
    /// Returns right direction
    fn right(&self) -> Vec2;
    /// Returns backward direction
    fn backward(&self) -> Vec2;

    /// Returns transform, with scale multiplied by `scale`
    fn scaled(self, scale_factor: Vec2) -> Self;
}

impl TransformExt for Transform {
    #[inline]
    fn forward(&self) -> Vec2 {
        let angle = QuatExt::to_angle(&self.rotation);
        Vec2::new(-angle.sin(), angle.cos())
    }

    #[inline]
    fn left(&self) -> Vec2 {
        let angle = QuatExt::to_angle(&self.rotation);
        Vec2::new(-angle.cos(), -angle.sin())
    }

    #[inline]
    fn backward(&self) -> Vec2 {
        let angle = QuatExt::to_angle(&self.rotation);
        Vec2::new(angle.sin(), -angle.cos())
    }

    #[inline]
    fn right(&self) -> Vec2 {
        let angle = QuatExt::to_angle(&self.rotation);
        Vec2::new(angle.cos(), angle.sin())
    }

    fn scaled(mut self, scale_factor: Vec2) -> Self {
        self.scale.x *= scale_factor.x;
        self.scale.y *= scale_factor.y;
        self
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use bevy::math::Quat;

    use crate::config::EPS;

    use super::*;

    #[test]
    fn dirs() {
        // transform is looking to the left
        let transform =
            Transform::from_rotation(Quat::from_rotation_z(PI * 0.5));

        assert!(Vec2::abs_diff_eq(
            &transform.forward(),
            Vec2::new(-1.0, 0.0),
            EPS
        ));
        assert!(Vec2.abs_diff_eq(&transform.left(), Vec2::new(0.0, -1.0), EPS));
        assert!(Vec2::abs_diff_eq(
            &transform.backward(),
            Vec2::new(1.0, 0.0),
            EPS
        ));
        assert!(Vec2::abs_diff_eq(
            &transform.right(),
            Vec2::new(0.0, 1.0),
            EPS
        ));
    }
}
