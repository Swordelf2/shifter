use bevy::math::Vec2;

use crate::config::physics;

/// Component, indicating that this entity can move and collide with colliders
#[derive(Clone, Debug)]
#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
pub struct DynamicObject {
    /// External acceleration, is not modified by the physics system
    pub accel: Vec2,

    /// Velocity
    pub(super) vel: Vec2,

    /// The square of the speed that this object can not exceed
    pub(super) max_vel_squared: f32,

    /// Friction_accel = -`vel` * `friction_coeff`
    pub(super) friction_coeff: f32,
}

impl DynamicObject {
    /// Returns a dynamic object with global maximum velocity.
    pub fn new() -> Self {
        Self {
            accel: Vec2::ZERO,
            vel: Vec2::ZERO,
            max_vel_squared: physics::GLOBAL_MAX_VEL * physics::GLOBAL_MAX_VEL,
            friction_coeff: 0.0,
        }
    }

    pub fn from_max_vel_and_friction_coeff(
        max_vel: f32,
        friction_coeff: f32,
    ) -> Self {
        assert!(
            max_vel <= physics::GLOBAL_MAX_VEL,
            "`max_vel` is greater than `GLOBAL_MAX_VEL`"
        );
        Self {
            max_vel_squared: max_vel * max_vel,
            friction_coeff,
            ..Self::new()
        }
    }
}
