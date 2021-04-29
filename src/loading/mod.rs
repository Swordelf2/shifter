//! Loading state must load all resources, and transition into the next state
//! upon completion
use crate::asset;
use crate::config::paths;
use crate::state::AppState;

use bevy::asset::{Asset, LoadState};
use bevy::prelude::*;

/// Resource, used to keep track of all assets being loaded. Temporary, deleted
/// upon exiting state
pub struct HandlesToCheck(Vec<HandleUntyped>);

// Loads an asset while adding it to `handles_to_check`
fn load_asset<T: Asset>(
    asset_path: &str,
    asset_server: &AssetServer,
    handles_to_check: &mut Vec<HandleUntyped>,
) -> Handle<T> {
    let handle = asset_server.load(asset_path);
    handles_to_check.push(handle.clone_untyped());
    handle
}

/* Systems */

/// Start loading assets
pub fn start_loading(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut handles_to_check: Vec<HandleUntyped> = Vec::new();
    // Textures
    {
        commands.insert_resource(asset::TextureHandles {
            player: load_asset(
                paths::textures::PLAYER,
                &asset_server,
                &mut handles_to_check,
            ),
            princess: load_asset(
                paths::textures::PRINCESS,
                &asset_server,
                &mut handles_to_check,
            ),
            maps: [load_asset(
                paths::textures::MAPS[0],
                &asset_server,
                &mut handles_to_check,
            )],
        });
    }
    // Fonts
    {
        commands.insert_resource(asset::FontHandles {
            noto_sans_regular: load_asset(
                paths::fonts::NOTO_SANS_REGULAR,
                &asset_server,
                &mut handles_to_check,
            ),
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

        commands.insert_resource(asset::MaterialHandles {
            player: materials
                .add(ColorMaterial::from(texture_handles.player.clone())),
            princess: materials
                .add(ColorMaterial::from(texture_handles.princess.clone())),
            hazard: materials
                .add(ColorMaterial::from(Color::rgb(0.0, 0.0, 0.95))),
            maps: [materials
                .add(ColorMaterial::from(texture_handles.maps[0].clone()))],
        });

        // Transition to the next state
        app_state.set(AppState::Menu).unwrap();
    }
}
