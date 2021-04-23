use std::f32::consts::PI;

use bevy::math::{Quat, Vec3};

use super::Vec3Ext;

/// Utility methods for 2d Quats/rotations
pub trait QuatExt {
    /// Get rotation angle, while checking that it's around Z axis
    fn to_angle(&self) -> f32;

    /// Rotate the quat around the Z axis counter-clockwise.
    /// Requires `0 <= delta < 2*PI`
    fn rotate(&mut self, delta: f32);
}

impl QuatExt for Quat {
    fn to_angle(&self) -> f32 {
        let (axis, angle) = self.to_axis_angle();
        assert!(
            angle == 0.0 || Vec3Ext::almost_eq(&axis, Vec3::new(0.0, 0.0, 1.0)),
            "Axis-angle is not around Z axis, maybe look at EPS\n\
            angle = {:?}, axis = {:?}",
            angle,
            axis
        );
        angle
    }

    fn rotate(&mut self, delta: f32) {
        assert!(0.0 <= delta && delta < 2.0 * PI);
        let angle = self.to_angle();
        let new_angle = angle + delta;
        let new_angle = if new_angle < 2.0 * PI {
            new_angle
        } else {
            new_angle - 2.0 * PI
        };
        *self = Quat::from_rotation_z(new_angle);
    }
}
