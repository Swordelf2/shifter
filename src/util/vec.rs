use bevy::math::{Vec2, Vec3};

const EPS: f32 = 1e-4;

pub trait Vec2Ext {
    /// Check whether the each component of the difference of the vectors is less than `EPS`
    fn almost_eq(&self, other: Vec2) -> bool;
}

impl Vec2Ext for Vec2 {
    fn almost_eq(&self, other: Vec2) -> bool {
        let diff = *self - other;
        diff.x < EPS && diff.y < EPS
    }
}

pub trait Vec3Ext {
    /// Check whether the each component of the difference of the vectors is less than `EPS`
    fn almost_eq(&self, other: Vec3) -> bool;
}

impl Vec3Ext for Vec3 {
    fn almost_eq(&self, other: Vec3) -> bool {
        let diff = *self - other;
        diff.x < EPS && diff.y < EPS && diff.z < EPS
    }
}
