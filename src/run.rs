use bevy::prelude::*;

use crate::{game, loading, menu, state::AppState};

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
    // Events
    .add_event::<game::spawn::Spawn>()
    /* Loading module */
    .add_system_set(
        SystemSet::on_enter(AppState::Loading)
            .with_system(loading::start_loading.system()),
    )
    .add_system_set(
        SystemSet::on_update(AppState::Loading)
            .with_system(loading::check_loading.system()),
    )
    ////* Menu module *////
    .add_system_set(
        SystemSet::on_enter(AppState::Menu).with_system(menu::setup.system()),
    )
    .add_system_set(
        SystemSet::on_update(AppState::Menu).with_system(menu::update.system()),
    )
    .add_system_set(
        SystemSet::on_exit(AppState::Menu).with_system(menu::exit.system()),
    )
    ////* Game module *////
    // Enter
    .add_system_set(
        SystemSet::on_enter(AppState::Game)
            .with_system(game::overlord::spawn_overlord.system())
            .with_system(game::world::spawn_world.system()),
    )
    // Input
    .add_system_set(
        SystemSet::on_update(AppState::Game)
            .label(game::SystemLabel::Input)
            .with_system(game::player::input.system())
            .with_system(game::overlord::exit_press.system()),
    )
    // Physics
    .add_system_set(
        SystemSet::on_update(AppState::Game)
            .label(game::SystemLabel::Physics)
            .after(game::SystemLabel::Input)
            .with_system(game::physics::movement.system()),
    )
    // Player rotation mechanic
    .add_system_set(
        SystemSet::on_update(AppState::Game)
            .with_system(game::player::rotation.system()),
    )
    // Spawn
    .add_system_set(
        SystemSet::on_update(AppState::Game)
            .with_system(game::spawn::spawn.system()),
    )
    // Exit
    .add_system_set(
        SystemSet::on_exit(AppState::Game)
            .with_system(game::overlord::exit.system()),
    );

    ////* Debug module *////
    #[cfg(feature = "debug")]
    {
        app.add_plugin(bevy_inspector_egui::WorldInspectorPlugin::new())
            .add_system(debug::test_system.system());
    }

    app.run();
}
