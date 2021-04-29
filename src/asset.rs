use bevy::prelude::*;

use crate::config::maps;

/// Handles to loaded textures
pub struct TextureHandles {
    pub player: Handle<Texture>,
    pub princess: Handle<Texture>,
    pub maps: [Handle<Texture>; maps::COUNT],
}

/// Handles to materials
pub struct MaterialHandles {
    pub player: Handle<ColorMaterial>,
    pub princess: Handle<ColorMaterial>,
    pub hazard: Handle<ColorMaterial>,
    pub maps: [Handle<ColorMaterial>; maps::COUNT],
}

/// Handles to fonts
pub struct FontHandles {
    pub noto_sans_regular: Handle<Font>,
}
