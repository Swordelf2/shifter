use bevy::prelude::*;
use image::RgbImage;

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

/// Pixel image, which is used to construct world map
pub struct MapImage(pub RgbImage);
