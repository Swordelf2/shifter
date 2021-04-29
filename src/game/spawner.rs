//! Spawn prefabs by sending Spawn event.
//! Prefabs are preconstructed customizable entities

use bevy::app::Events;
use bevy::prelude::*;

use crate::asset::MaterialHandles;
use crate::config::{depths, CELL_SIZE, PLAYER_MAX_SPEED};

use super::{overlord, physics, player};

/// Spawn type, indicating which prefab to spawn
pub enum Prefab {
    Player,
    Princess,
    WorldMap(WorldMapPrefab),
}

/// WorldMap prefab, containing the world map's info, required for
/// initializing, such as map id
pub struct WorldMapPrefab {
    pub map_id: usize,
}

/// Spawn event, containing all common data between prefabs and the prefab itself
pub struct Spawn {
    pub position: Vec2,
    pub prefab: Prefab,
}

/// Read (consume) spawn events and spawn corresponding prefabs
pub fn spawn(
    mut commands: Commands,
    mut spawns: ResMut<Events<Spawn>>,
    material_handles: Res<MaterialHandles>,
    overlord_query: Query<Entity, With<overlord::Overlord>>,
) {
    // Everything that spawns in the `game` should be inserted into overlord's children
    let mut overlord_new_children = Vec::new();
    for spawn in spawns.drain() {
        let new_child = match spawn.prefab {
            Prefab::Player => commands
                .spawn_bundle(SpriteBundle {
                    material: material_handles.player.clone(),
                    transform: Transform::from_translation(Vec3::from((
                        spawn.position,
                        depths::PLAYER,
                    ))),
                    sprite: Sprite::new(Vec2::new(
                        2.0 * CELL_SIZE,
                        2.0 * CELL_SIZE,
                    )),
                    ..Default::default()
                })
                .insert(player::Player)
                .insert(physics::DynamicObject::new(Some(PLAYER_MAX_SPEED)))
                .id(),
            Prefab::Princess => {
                commands
                    .spawn_bundle(SpriteBundle {
                        material: material_handles.princess.clone(),
                        transform: Transform::from_translation(Vec3::from((
                            spawn.position,
                            depths::PRINCESS,
                        ))),
                        sprite: Sprite::new(Vec2::new(CELL_SIZE, CELL_SIZE)),
                        ..Default::default()
                    })
                    .id()
                //.insert(Princess or smth)
            }
            Prefab::WorldMap(world_map_prefab) => commands
                .spawn_bundle(SpriteBundle {
                    material: material_handles.maps[world_map_prefab.map_id]
                        .clone(),
                    transform: Transform::from_translation(Vec3::from((
                        spawn.position,
                        depths::WORLD_MAP,
                    ))),
                    sprite: Sprite::new(Vec2::new(1600.0, 1600.0)),
                    ..Default::default()
                })
                .id(),
        };

        overlord_new_children.push(new_child);
    }

    commands
        .entity(overlord_query.single().unwrap())
        .push_children(&overlord_new_children);
}
