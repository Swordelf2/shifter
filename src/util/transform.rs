use bevy::{
    math::{Vec2, Vec3},
    transform::components::Transform,
};

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

    /// Adds `translation` to `self`
    fn translate(&mut self, translation: Vec2);

    /// Sets x and y components of the translation to `translation`
    fn translate_to(&mut self, translation: Vec2);
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

    #[inline]
    fn translate(&mut self, translation: Vec2) {
        self.translation += Vec3::from((translation, 0.0));
    }

    #[inline]
    fn translate_to(&mut self, translation: Vec2) {
        self.translation = Vec3::from((translation, self.translation.z));
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use bevy::math::Quat;

    use super::*;

    #[test]
    fn dirs() {
        const EPS: f32 = 1e-6;
        // transform is looking to the left
        let transform =
            Transform::from_rotation(Quat::from_rotation_z(PI * 0.5));

        assert!(Vec2::abs_diff_eq(
            TransformExt::forward(&transform),
            Vec2::new(-1.0, 0.0),
            EPS
        ));
        assert!(Vec2::abs_diff_eq(
            TransformExt::left(&transform),
            Vec2::new(0.0, -1.0),
            EPS
        ));
        assert!(Vec2::abs_diff_eq(
            TransformExt::backward(&transform),
            Vec2::new(1.0, 0.0),
            EPS
        ));
        assert!(Vec2::abs_diff_eq(
            TransformExt::right(&transform),
            Vec2::new(0.0, 1.0),
            EPS
        ));
    }
}
