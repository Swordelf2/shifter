use bevy::prelude::*;

use crate::menu;
use crate::util::TransformExt;

use super::prefab::Prefabs;
/* Systems*/

/// System, that initializes the whole world by sending [Spawn] events
pub fn spawn(
    mut commands: Commands,
    map_button: Res<menu::MapButton>,
    prefabs: Res<Prefabs>,
) {
    commands.spawn_bundle(prefabs.world_map1.clone());

    commands.spawn_bundle(prefabs.player.clone());

    let mut princess = prefabs.princess.clone();
    princess
        .sprite_bundle
        .transform
        .translate_to(Vec2::new(200.0, 200.0));
    commands.spawn_bundle(princess);
}
