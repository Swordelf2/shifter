/// Asset paths
pub mod paths {
    pub mod textures {
        pub const PLAYER: &str = "textures/player.png";
        pub const PRINCESS: &str = "textures/princess.png";
        pub const MAPS: [&str; crate::config::maps::COUNT] = ["maps/map1.png"];
    }

    pub mod fonts {
        pub const NOTO_SANS_REGULAR: &str = "fonts/NotoSans-Regular.ttf";
    }
}

pub mod maps {
    pub const COUNT: usize = 1;
}

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
pub const ROTATION_SPEED: f32 = 1.0;

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
