use bevy::{math::Vec2, transform::components::Transform};

use super::QuatExt;

/// Utility methods for 2D `Transform`s
/// TODO: nothing here is used yet, maybe remove
pub trait TransformExt {
    /// Return forward direction
    fn forward(&self) -> Vec2;
    fn left(&self) -> Vec2;
    fn right(&self) -> Vec2;
    fn backward(&self) -> Vec2;
}

impl TransformExt for Transform {
    fn forward(&self) -> Vec2 {
        let angle = QuatExt::to_angle(&self.rotation);
        Vec2::new(-angle.sin(), angle.cos())
    }

    fn left(&self) -> Vec2 {
        let angle = QuatExt::to_angle(&self.rotation);
        Vec2::new(-angle.cos(), -angle.sin())
    }

    fn backward(&self) -> Vec2 {
        let angle = QuatExt::to_angle(&self.rotation);
        Vec2::new(angle.sin(), -angle.cos())
    }

    fn right(&self) -> Vec2 {
        let angle = QuatExt::to_angle(&self.rotation);
        Vec2::new(angle.cos(), angle.sin())
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use bevy::math::Quat;

    use crate::util::Vec2Ext;

    use super::*;

    #[test]
    fn dirs() {
        // transform is looking to the left
        let transform =
            Transform::from_rotation(Quat::from_rotation_z(PI * 0.5));

        assert!(Vec2Ext::almost_eq(
            &transform.forward(),
            Vec2::new(-1.0, 0.0)
        ));
        assert!(Vec2Ext::almost_eq(&transform.left(), Vec2::new(0.0, -1.0)));
        assert!(Vec2Ext::almost_eq(
            &transform.backward(),
            Vec2::new(1.0, 0.0)
        ));
        assert!(Vec2Ext::almost_eq(&transform.right(), Vec2::new(0.0, 1.0)));
    }
}
