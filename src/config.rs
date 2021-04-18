/// Asset paths
pub mod paths {
    pub const MAP: &str = "assets/map.png";

    pub mod textures {
        pub const PLAYER: &str = "textures/player.png";
        pub const PRINCESS: &str = "textures/princess.png";
    }
}

/// Mapping from in-game entities to colors on the map image
pub mod map_colors {
    pub const PRINCESS: [u8; 3] = [255, 0, 0];
    pub const PLAYER: [u8; 3] = [0, 255, 0];
    pub const HAZARD: [u8; 3] = [0, 0, 255];
}

// Cell size in pixels
pub const CELL_SIZE: f32 = 32.0;
// Player move speed
pub const MOVE_SPEED: f32 = CELL_SIZE * 3.0;
// Player rotation speed
pub const ROTATION_SPEED: f32 = 0.0; // 1.0

/// Z depths of entities
pub mod depths {
    pub const PLAYER: f32 = 0.1;
    pub const PRINCESS: f32 = 0.1;
    pub const HAZARD: f32 = 0.0;
}
