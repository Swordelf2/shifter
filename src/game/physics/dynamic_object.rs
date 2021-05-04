use bevy::math::Vec2;

use crate::config::physics;

/// Component, indicating that this entity can move and collide with colliders
#[derive(Debug)]
#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
pub struct DynamicObject {
    /// Aceleration
    pub accel: Vec2,

    /// Velocity
    pub(super) vel: Vec2,

    /// The square of the speed that this object can not exceed
    pub(super) max_vel_squared: f32,
}

impl DynamicObject {
    /// Returns a dynamic object with global maximum velocity.
    pub fn new() -> Self {
        Self {
            accel: Default::default(),
            vel: Default::default(),
            max_vel_squared: physics::GLOBAL_MAX_VEL * physics::GLOBAL_MAX_VEL,
        }
    }

    pub fn from_max_vel(max_vel: f32) -> Self {
        assert!(
            max_vel <= physics::GLOBAL_MAX_VEL,
            "`max_vel` is greater than `GLOBAL_MAX_VEL`"
        );
        Self {
            max_vel_squared: max_vel * max_vel,
            ..Self::new()
        }
    }
}
