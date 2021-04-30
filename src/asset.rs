use std::collections::HashMap;
use std::path::PathBuf;

use bevy::prelude::*;

use strum::EnumIter;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, EnumIter)]
pub enum ObjectLabel {
    Player,
    Princess,
    WorldMap1,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, EnumIter)]
pub enum FontLabel {
    NotoSansRegular,
}

pub fn object_label_to_path(object_label: ObjectLabel) -> PathBuf {
    PathBuf::from(match object_label {
        ObjectLabel::Player => "textures/player",
        ObjectLabel::Princess => "textures/princess",
        ObjectLabel::WorldMap1 => "maps/map1",
    })
}

pub fn font_label_to_path(font_label: FontLabel) -> PathBuf {
    PathBuf::from(match font_label {
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
