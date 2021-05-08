use bevy::prelude::*;

use crate::menu;

use super::spawner::{Prefab, Spawn, WorldMapPrefab};

/* Systems*/

/// System, that initializes the whole world by sending [Spawn] events
pub fn spawn(map_button: Res<menu::MapButton>, mut spawns: EventWriter<Spawn>) {
    // Spawn the map at (0, 0)
    spawns.send(Spawn {
        position: Vec2::ZERO,
        prefab: Prefab::WorldMap(WorldMapPrefab {
            map_id: map_button.map_id,
        }),
    });
    // Spawn the player at (0, 0)
    spawns.send(Spawn {
        position: Vec2::ZERO,
        prefab: Prefab::Player,
    });
    // Spawn the princess
    spawns.send(Spawn {
        position: Vec2::new(150.0, 150.0),
        prefab: Prefab::Princess,
    });
}
