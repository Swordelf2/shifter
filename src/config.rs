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

/// Cell size in pixels
pub const CELL_SIZE: f32 = 32.0;
/// Player acceleration
pub const PLAYER_ACCEL: f32 = 5.0 * CELL_SIZE;
/// Player maximum speed
pub const PLAYER_MAX_SPEED: f32 = 10.0 * CELL_SIZE;
/// Player rotation speed
pub const ROTATION_SPEED: f32 = 0.0;

pub const EPS: f32 = 1e-6;

pub mod physics {
    /// No physical object in the game can exceed this speed
    /// All speeds are in (world points)/second
    pub const GLOBAL_MAX_SPEED: f32 = 200.0;
}

/// Z depths of entities
pub mod depths {
    pub const PLAYER: f32 = 0.1;
    pub const PRINCESS: f32 = 0.1;
    pub const WORLD_MAP: f32 = 0.0;
}

/// These constants are used for svg conversions
pub mod conversion {
    const PX_PER_INCH: f32 = 96.0;
    const MM_PER_INCH: f32 = 25.4;
    const PX_PER_MM: f32 = PX_PER_INCH / MM_PER_INCH;
    /// Multiply svg coords by this constant to get in-app units
    pub const SVG_TO_UNITS: f32 = PX_PER_MM;
}
