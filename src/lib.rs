/*
mod loading;
mod menu;
mod game;
mod states;
*/
#[cfg(feature = "debug")]
mod debug;

use bevy::prelude::*;

#[derive(Debug)]
pub struct TestComponent(u8);

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
    // Default plugins
    .add_plugins(DefaultPlugins)
    .add_startup_system(test_startup_system.system());
    /*
    // Startup system
    .add_startup_system(setup::setup.system())
    // Systems
    .add_system(systems::animation.system())
    .add_system(systems::player_control.system())
    //.add_system(systems::camera_movement.system())
    .add_system(systems::collision.system())
    .add_system(systems::score.system());
        */

    /*
    #[cfg(feature = "debug")]
    app.add_system(debug::test_system.system());
    */


    app.run();
}

fn test_startup_system(mut commands: Commands) {
    commands.spawn().insert(TestComponent(5));

    commands.spawn().insert(TestComponent(8));
}
