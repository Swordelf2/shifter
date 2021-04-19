//! Spawn prefabs by sending Spawn event.
//! Prefabs are preconstructed customizable entities

use bevy::app::Events;
use bevy::prelude::*;

use crate::asset::MaterialHandles;
use crate::config::{depths, CELL_SIZE};

use super::player::Player;

/// Spawn type, indicating which prefab to spawn
pub enum Prefab {
    Player(PlayerPrefab),
    Princess,
    Hazard,
}

/// Player prefab, with a camera attached to it by default
pub struct PlayerPrefab {
    with_camera: bool,
}

impl Default for PlayerPrefab {
    fn default() -> Self {
        Self { with_camera: true }
    }
}

/// Spawn event, containing all common data between prefabs and the prefab itself
pub struct Spawn {
    // Transform components
    pub position: Vec2,

    pub prefab: Prefab,
}

/// Read (consume) spawn events and spawn corresponding prefabs
pub fn spawn(
    mut commands: Commands,
    mut spawns: ResMut<Events<Spawn>>,
    material_handles: Res<MaterialHandles>,
) {
    for spawn in spawns.drain() {
        match spawn.prefab {
            Prefab::Player(player_prefab) => {
                let mut entity = commands.spawn_bundle(SpriteBundle {
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
                });
                entity.insert(Player);
                if player_prefab.with_camera {
                    // Spawn camera as a child to the player
                    entity.with_children(|parent| {
                        let mut camera = OrthographicCameraBundle::new_2d();
                        // absolute z should be = `far` - `eps`
                        // (camera only sees 1000.0 on z axis forward for some reason)
                        camera.transform.translation.z =
                            1000.0 - 0.5 - depths::PLAYER;
                        parent.spawn_bundle(camera);
                    });
                }
            }
            Prefab::Princess => {
                commands.spawn_bundle(SpriteBundle {
                    material: material_handles.princess.clone(),
                    transform: Transform::from_translation(Vec3::from((
                        spawn.position,
                        depths::PRINCESS,
                    ))),
                    sprite: Sprite::new(Vec2::new(CELL_SIZE, CELL_SIZE)),
                    ..Default::default()
                });
                //.insert(Princess or smth)
            }
            Prefab::Hazard => {
                commands.spawn_bundle(SpriteBundle {
                    material: material_handles.hazard.clone(),
                    transform: Transform::from_translation(Vec3::from((
                        spawn.position,
                        depths::HAZARD,
                    ))),
                    sprite: Sprite::new(Vec2::new(CELL_SIZE, CELL_SIZE)),
                    ..Default::default()
                });
                //.insert(Hazard or smth)
            }
        }
    }
}
