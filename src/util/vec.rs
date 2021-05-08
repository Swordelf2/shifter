use bevy::math::{Vec2, Vec3};
use bevy::transform::components::Transform;

pub trait Vec2Ext {
    /// Check whether the each component of the difference of the vectors is less than `EPS`
    fn collinear(self, other: Self) -> bool;

    /// Apply transform to a 2d point
    fn apply_transform(self, transform: &Transform) -> Self;

    /// Returns a vector, rotated clockwise by `PI/2`
    fn rotate_clockwise_90(self) -> Self;

    /// Reflects the vector along a given (not necessarily normalized) normal
    fn reflect(self, normal: Self) -> Self;
}

impl Vec2Ext for Vec2 {
    fn collinear(self, other: Vec2) -> bool {
        const EPS: f32 = 1e-6;
        Vec2::angle_between(self, other).abs() < EPS
    }

    fn apply_transform(self, transform: &Transform) -> Self {
        (*transform * Vec3::from((self, 0.0))).truncate()
    }

    fn rotate_clockwise_90(self) -> Vec2 {
        Vec2::new(self.y, -self.x)
    }

    fn reflect(self, normal: Vec2) -> Vec2 {
        let falling = self;
        falling - 2.0 * falling.dot(normal) / normal.length_squared() * normal
    }
}

pub trait Vec3Ext {
    /// Check whether the each component of the difference of the vectors is less than `EPS`
    fn collinear(self, other: Self) -> bool;
}

impl Vec3Ext for Vec3 {
    fn collinear(self, other: Self) -> bool {
        const EPS: f32 = 1e-6;
        Vec3::angle_between(self, other).abs() < EPS
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_clockwise_90_test() {
        const EPS: f32 = 1e-7;
        assert!(Vec2::abs_diff_eq(
            Vec2::new(1.0, 1.0).rotate_clockwise_90(),
            Vec2::new(1.0, -1.0),
            EPS
        ));
        assert!(Vec2::abs_diff_eq(
            Vec2::new(-2.0, 1.0).rotate_clockwise_90(),
            Vec2::new(1.0, 2.0),
            EPS
        ));
    }

    #[test]
    fn reflect_test() {
        const EPS: f32 = 1e-7;
        assert!(Vec2::abs_diff_eq(
            Vec2::new(3.0, 3.0).reflect(Vec2::new(0.0, -1.0)),
            Vec2::new(3.0, -3.0),
            EPS
        ));
        assert!(Vec2::abs_diff_eq(
            Vec2::new(-100.0, 0.0).reflect(Vec2::new(0.002, 0.002)),
            Vec2::new(0.0, 100.0),
            EPS
        ));
    }
}
