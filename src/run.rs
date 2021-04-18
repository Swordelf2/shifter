use bevy::prelude::*;

use crate::{game, loading, state::AppState};

#[cfg(feature = "debug")]
use crate::debug;

pub fn run() {
    let mut app = App::build();

    app.insert_resource(WindowDescriptor {
        title: "bevygame".to_string(),
        width: 1024.0,
        height: 1024.0,
        vsync: false,
        resizable: false,
        ..Default::default()
    })
    .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
    // Default plugins
    .add_plugins(DefaultPlugins)
    // State //
    .add_state(AppState::Loading)
    /* Loading module */
    .add_system_set(
        SystemSet::on_enter(AppState::Loading)
            .with_system(loading::start_loading.system()),
    )
    .add_system_set(
        SystemSet::on_update(AppState::Loading)
            .with_system(loading::check_loading.system()),
    )
    /* Menu module */
    // TODO
    /* Game module */
    .add_system_set(
        SystemSet::on_enter(AppState::Game)
            .with_system(game::spawn_world.system()),
    )
    .add_system_set(
        SystemSet::on_update(AppState::Game)
            .with_system(game::player::player_control.system()),
    );

    /* Debug module */
    #[cfg(feature = "debug")]
    app.add_system(debug::test_system.system());

    app.run();
}
