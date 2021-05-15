//! Game state is responsible for the actual gameplay of the app
pub mod physics;
pub mod player;
pub mod prefab;
pub mod world;

mod label;

pub use label::SystemLabel;

use bevy::prelude::*;
use bevy::render::camera::Camera;

use crate::config::keybinds;
use crate::state::AppState;

/// System, responsible for exiting to main menu, i.e. changing app state
pub fn exit_press(
    mut input: ResMut<Input<KeyCode>>,
    mut state: ResMut<State<AppState>>,
) {
    if input.just_pressed(keybinds::PAUSE) {
        state.set(AppState::Menu).unwrap();
        input.reset(keybinds::PAUSE);
    }
}

pub fn exit(
    mut commands: Commands,
    game_entities_query: Query<Entity, Without<Camera>>,
) {
    // Despawn the overlord and all its children
    for entity in game_entities_query.iter() {
        commands.entity(entity).despawn();
    }
}
