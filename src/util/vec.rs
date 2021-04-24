use bevy::math::{Vec2, Vec3};

use crate::config::EPS;

pub trait Vec2Ext {
    /// Check whether the each component of the difference of the vectors is less than `EPS`
    fn collinear(self, other: Self) -> bool;
}

impl Vec2Ext for Vec2 {
    fn collinear(self, other: Self) -> bool {
        Vec2::angle_between(self, other).abs() < EPS
    }
}

pub trait Vec3Ext {
    /// Check whether the each component of the difference of the vectors is less than `EPS`
    fn collinear(self, other: Self) -> bool;
}

impl Vec3Ext for Vec3 {
    fn collinear(self, other: Self) -> bool {
        Vec3::angle_between(self, other).abs() < EPS
    }
}
