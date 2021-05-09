use std::f32::consts::PI;

/// Controls
pub mod keybinds {
    use bevy::input::keyboard::KeyCode;

    pub const PAUSE: KeyCode = KeyCode::Escape;

    pub mod movement {
        use super::KeyCode;

        pub const UP: KeyCode = KeyCode::W;
        pub const RIGHT: KeyCode = KeyCode::D;
        pub const LEFT: KeyCode = KeyCode::A;
        pub const DOWN: KeyCode = KeyCode::S;
    }
}

/// Player rotation speed (in radians/s)
pub const ROTATION_SPEED: f32 = 0.0 * (PI * 2.0); // 0.25

pub mod physics {
    /// No physical object in the game can exceed this speed
    /// All speeds are in (world points)/second
    pub const GLOBAL_MAX_VEL: f32 = 500.0;
    /// Player acceleration
    pub const PLAYER_ACCEL: f32 = 700.0; // 1000.0
    /// Player maximum speed
    pub const PLAYER_MAX_VEL: f32 = 320.0; // 320.0
    /// Player friction coefficient
    pub const PLAYER_FRICTION_COEFF: f32 = 1.0; // 5.0

    pub const BOUNCINESS: f32 = 1.0;
}

/// Z depths of entities
pub mod depths {
    pub const PLAYER: f32 = 2.0;
    pub const PRINCESS: f32 = 1.0;
    pub const WORLD_MAP: f32 = 0.0;
}

/// Sizes of entities
pub mod sizes {
    use bevy::math::{const_vec2, Vec2};

    pub const PLAYER: Vec2 = const_vec2!([64.0, 128.0]);
    pub const PRINCESS: Vec2 = const_vec2!([200.0, 200.0]);
    pub const WORLD_MAP1: Vec2 = const_vec2!([1600.0, 1600.0]);
}

/// These constants are used for svg conversions
pub mod conversion {
    const PX_PER_INCH: f32 = 96.0;
    const MM_PER_INCH: f32 = 25.4;
    const PX_PER_MM: f32 = PX_PER_INCH / MM_PER_INCH;
    /// Multiply svg coords by this constant to get in-app units
    pub const SVG_TO_UNITS: f32 = PX_PER_MM;
}
