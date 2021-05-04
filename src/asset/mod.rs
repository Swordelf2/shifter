pub mod svgdata;

use std::collections::HashMap;
use std::path::Path;

use bevy::prelude::*;

use strum::{EnumCount, EnumIter};

pub use svgdata::SvgData;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, EnumCount, EnumIter)]
pub enum ObjectLabel {
    Player,
    Princess,
    WorldMap1,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, EnumCount, EnumIter)]
pub enum FontLabel {
    NotoSansRegular,
}

pub fn object_label_to_texture_path(
    object_label: ObjectLabel,
) -> &'static Path {
    Path::new(match object_label {
        ObjectLabel::Player => "textures/player_square.png",
        ObjectLabel::Princess => "textures/princess.png",
        ObjectLabel::WorldMap1 => "textures/map1.png",
    })
}

pub fn object_label_to_svg_path(
    object_label: ObjectLabel,
) -> Option<&'static Path> {
    match object_label {
        ObjectLabel::Player => Some(Path::new("textures/player_square.svg")),
        ObjectLabel::Princess => Some(Path::new("textures/princess.svg")),
        ObjectLabel::WorldMap1 => Some(Path::new("textures/map1.svg")),
    }
}

pub fn font_label_to_path(font_label: FontLabel) -> &'static Path {
    Path::new(match font_label {
        FontLabel::NotoSansRegular => "fonts/NotoSans-Regular.ttf",
    })
}

/// Handles to loaded textures
pub struct TextureHandles {
    pub handles: HashMap<ObjectLabel, Handle<Texture>>,
}

/// Handles to materials
pub struct MaterialHandles {
    pub handles: HashMap<ObjectLabel, Handle<ColorMaterial>>,
}

/// Handles to fonts
pub struct FontHandles {
    pub handles: HashMap<FontLabel, Handle<Font>>,
}

/// Handles to svg data
pub struct SvgDataHandles {
    pub handles: HashMap<ObjectLabel, Handle<SvgData>>,
}
