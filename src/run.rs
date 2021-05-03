use bevy::prelude::*;

use crate::{asset, camera, game, loading, menu, state::AppState};

#[cfg(feature = "debug")]
use crate::debug;

pub fn run() {
    let mut app = App::build();

    app.insert_resource(WindowDescriptor {
        title: "bevygame".to_string(),
        width: 800.0,
        height: 800.0,
        vsync: false,
        resizable: false,
        ..Default::default()
    })
    .insert_resource(ClearColor(Color::rgb(
        0x79 as f32 / 255.0,
        0xc0 as f32 / 255.0,
        0xe9 as f32 / 255.0,
    )))
    // Default plugins
    .add_plugins(DefaultPlugins)
    // SvgData custom asset and loader
    .add_asset::<asset::SvgData>()
    .init_asset_loader::<asset::svgdata::SvgDataLoader>()
    // State //
    .add_state(AppState::Loading)
    // Events
    .add_event::<game::spawner::Spawn>()
    /*** SYSTEMS ***/
    // Startup systems
    .add_startup_system(camera::spawn.system())
    ////* Loading module *////
    .add_system_set(
        SystemSet::on_enter(AppState::Loading)
            .with_system(loading::start_loading_assets.system()),
    )
    .add_system_set(
        SystemSet::on_update(AppState::Loading)
            .with_system(loading::check_loading_assets.system()),
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
            .with_system(game::overlord::spawn.system())
            .with_system(game::world::spawn.system()),
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
            .with_system(game::physics::update.system()),
    )
    // Camera movement
    .add_system_set(
        SystemSet::on_update(AppState::Game)
            .after(game::SystemLabel::Physics)
            .with_system(camera::movement.system()),
    )
    // Player rotation mechanic
    .add_system_set(
        SystemSet::on_update(AppState::Game)
            .with_system(game::player::rotation.system()),
    )
    // Spawner
    .add_system_set(
        SystemSet::on_update(AppState::Game)
            .with_system(game::spawner::spawn.system()),
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

        // Register components with `bevy_inspector_egui`
        let mut registry = app.world_mut().get_resource_or_insert_with(
            bevy_inspector_egui::InspectableRegistry::default,
        );
        registry.register::<game::physics::DynamicObject>();
        registry.register::<game::physics::Collider>();
    }

    app.run();
}
