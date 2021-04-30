//! Loading state must load all resources, and transition into the next state
//! upon completion
use crate::asset;
use crate::state::AppState;

use bevy::asset::LoadState;
use bevy::prelude::*;

use strum::IntoEnumIterator;

/// Resource, used to keep track of all assets being loaded. Temporary, deleted
/// upon exiting state
pub struct HandlesToCheck(Vec<HandleUntyped>);

/* Systems */

/// Start loading assets
pub fn start_loading(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut handles_to_check: Vec<HandleUntyped> = Vec::new();
    // Textures
    // Iterate over all object labels and load their texture
    commands.insert_resource(asset::TextureHandles {
        handles: asset::ObjectLabel::iter()
            .map(|object_label| {
                let mut path = asset::object_label_to_path(object_label);
                path.set_extension("png");
                let handle = asset_server.load(path);
                handles_to_check.push(handle.clone_untyped());
                (object_label, handle)
            })
            .collect(),
    });

    // Fonts
    // Iterate over all font labels and load the font
    {
        commands.insert_resource(asset::FontHandles {
            handles: asset::FontLabel::iter()
                .map(|font_label| {
                    let path = asset::font_label_to_path(font_label);
                    let handle = asset_server.load(path);
                    handles_to_check.push(handle.clone_untyped());
                    (font_label, handle)
                })
                .collect(),
        });
    }

    commands.insert_resource(HandlesToCheck(handles_to_check));

    // TODO load the svg from maps probably here
}

/// Check if all assets are loaded
pub fn check_loading(
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
