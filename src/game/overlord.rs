//! Overlord is the entity that controls the game. It is the root of all
//! non-camera entities in the `Game` state.
use bevy::prelude::*;

use crate::config::keybinds;
use crate::state::AppState;

/// Marker component for the overlord entity
pub struct Overlord;

pub fn spawn(mut commands: Commands) {
    commands.spawn_bundle((
        Transform::default(),
        GlobalTransform::default(),
        Overlord,
    ));
}

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
    overlord_query: Query<Entity, With<Overlord>>,
) {
    // Despawn the overlord and all its children
    commands
        .entity(overlord_query.single().unwrap())
        .despawn_recursive();
}
