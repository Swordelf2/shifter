//! Loading state must load all resources, and transition into the next state
//! upon completion
use crate::asset;
use crate::config::paths;
use crate::state::AppState;

use bevy::asset::LoadState;
use bevy::prelude::*;

/// Resource, used to keep track of all assets being loaded. Temporary, deleted
/// upon exiting state
pub struct HandlesToCheck(Vec<HandleUntyped>);

/* Systems */

/// Start loading assets
pub fn start_loading(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Textures
    let mut handles: Vec<HandleUntyped> = Vec::new();

    let player = asset_server.load(paths::textures::PLAYER);
    handles.push(player.clone_untyped());
    let princess = asset_server.load(paths::textures::PRINCESS);
    handles.push(princess.clone_untyped());

    commands.insert_resource(HandlesToCheck(handles));
    commands.insert_resource(asset::TextureHandles { player, princess });

    // Map image
    commands.insert_resource(asset::MapImage(
        image::open(paths::MAP).unwrap().into_rgb8(),
    ));
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
        // Transition to the next state
        app_state.set(AppState::Game).unwrap();
    }

    commands.insert_resource(asset::MaterialHandles {
        player: materials.add(ColorMaterial::from(texture_handles.player.clone())),
        princess: materials.add(ColorMaterial::from(texture_handles.princess.clone())),
        hazard: materials.add(ColorMaterial::from(Color::rgb(0.0, 0.0, 0.95))),
    });
}
