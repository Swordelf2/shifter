//! Loading state must load all resources, and transition into the next state
//! upon completion
use std::collections::HashMap;

use crate::asset;
use crate::state::AppState;

use bevy::asset::LoadState;
use bevy::prelude::*;

use strum::{EnumCount, IntoEnumIterator};

/// Resource, used to keep track of all assets being loaded. Temporary, deleted
/// upon exiting state
pub struct HandlesToCheck(Vec<HandleUntyped>);

/* Systems */

/// Start loading assets
pub fn start_loading_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let mut handles_to_check: Vec<HandleUntyped> = Vec::new();
    // Textures and svg data
    // Iterate over all object labels and load their texture and svg data
    let mut texture_handles: HashMap<asset::ObjectLabel, Handle<Texture>> =
        HashMap::with_capacity(asset::ObjectLabel::COUNT);
    let mut svg_data_handles: HashMap<
        asset::ObjectLabel,
        Handle<asset::SvgData>,
    > = HashMap::with_capacity(asset::ObjectLabel::COUNT);
    for object_label in asset::ObjectLabel::iter() {
        let texture_handle = asset_server
            .load(asset::object_label_to_texture_path(object_label));
        handles_to_check.push(texture_handle.clone_untyped());
        texture_handles.insert(object_label, texture_handle);

        if let Some(svg_data_path) =
            asset::object_label_to_svg_path(object_label)
        {
            let svg_data_handle = asset_server.load(svg_data_path);
            handles_to_check.push(svg_data_handle.clone_untyped());
            svg_data_handles.insert(object_label, svg_data_handle);
        }
    }
    commands.insert_resource(asset::TextureHandles {
        handles: texture_handles,
    });
    commands.insert_resource(asset::SvgDataHandles {
        handles: svg_data_handles,
    });

    // Fonts
    let mut font_handles: HashMap<asset::FontLabel, Handle<Font>> =
        HashMap::with_capacity(asset::FontLabel::COUNT);
    for font_label in asset::FontLabel::iter() {
        let font_handle =
            asset_server.load(asset::font_label_to_path(font_label));
        handles_to_check.push(font_handle.clone_untyped());
        font_handles.insert(font_label, font_handle);
    }
    commands.insert_resource(asset::FontHandles {
        handles: font_handles,
    });

    commands.insert_resource(HandlesToCheck(handles_to_check));
}

/// Check if all assets are loaded
pub fn check_loading_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    handles_to_check: Res<HandlesToCheck>,
    texture_handles: Res<asset::TextureHandles>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut app_state: ResMut<State<AppState>>,
) {
    if asset_server
        .get_group_load_state(handles_to_check.0.iter().map(|handle| handle.id))
        == LoadState::Loaded
    {
        commands.remove_resource::<HandlesToCheck>();

        // Iterate over all object labels and for each of them
        // create a material with their texture handle and put it into
        // `material_handles`
        commands.insert_resource(asset::MaterialHandles {
            handles: texture_handles
                .handles
                .iter()
                .map(|(&object_label, texture_handle)| {
                    (
                        object_label,
                        materials
                            .add(ColorMaterial::from(texture_handle.clone())),
                    )
                })
                .collect(),
        });

        // Transition to the next state
        app_state.set(AppState::Menu).unwrap();
    }
}
