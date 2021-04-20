use bevy::prelude::*;
use image::RgbImage;

use crate::config::paths;

/// Handles to loaded textures
pub struct TextureHandles {
    pub player: Handle<Texture>,
    pub princess: Handle<Texture>,
}

/// Handles to materials
pub struct MaterialHandles {
    pub player: Handle<ColorMaterial>,
    pub princess: Handle<ColorMaterial>,
    pub hazard: Handle<ColorMaterial>,
}

/// Handles to fonts
pub struct FontHandles {
    pub noto_sans_regular: Handle<Font>,
}

/// Pixel images, which are used to construct world map
pub struct MapImages {
    pub images: [RgbImage; paths::MAPS.len()],
}
