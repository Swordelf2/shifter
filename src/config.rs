/// Asset paths
pub mod paths {
    pub const MAP: &str = "assets/map.png";

    pub mod textures {
        pub const PLAYER: &str = "textures/player.png";
        pub const PRINCESS: &str = "textures/princess.png";
    }

    pub mod fonts {
        pub const NOTO_SANS_REGULAR: &str = "fonts/NotoSans-Regular.ttf";
    }
}

pub mod map {
    use crate::game::spawn::Prefab;

    pub fn pixel2prefab(pixel: [u8; 3]) -> Option<Prefab> {
        match pixel {
            // Red = Princess
            [255, 0, 0] => Some(Prefab::Princess),
            [0, 255, 0] => Some(Prefab::Player(Default::default())),
            [0, 0, 255] => Some(Prefab::Hazard),
            _ => None,
        }
    }
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
