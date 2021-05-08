//! Spawn prefabs by sending Spawn event.
//! Prefabs are preconstructed customizable entities

use bevy::app::Events;
use bevy::prelude::*;

use crate::asset;
use crate::asset::svgdata;
use crate::config;
use crate::config::{depths, sizes};
use crate::util::TransformExt;

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
    material_handles: Res<asset::MaterialHandles>,
    svg_datas: Res<Assets<asset::SvgData>>,
    svg_data_handles: Res<asset::SvgDataHandles>,
    overlord_query: Query<Entity, With<overlord::Overlord>>,
) {
    // Everything that spawns in the `game` should be inserted into overlord's children
    let mut overlord_new_children = Vec::new();
    for spawn in spawns.drain() {
        let new_child = match spawn.prefab {
            Prefab::Player => {
                let svg_data = svg_datas
                    .get(&svg_data_handles.handles[&asset::ObjectLabel::Player])
                    .unwrap();
                commands
                    .spawn_bundle(SpriteBundle {
                        material: material_handles.handles
                            [&asset::ObjectLabel::Player]
                            .clone(),
                        transform: Transform::from_translation(Vec3::from((
                            spawn.position,
                            depths::PLAYER,
                        )))
                        .scaled(sizes::PLAYER / svg_data.size),
                        sprite: Sprite::new(svg_data.size),
                        ..Default::default()
                    })
                    .insert(player::Player)
                    .insert(
                        physics::DynamicObject::from_max_vel_and_friction_coeff(
                            config::physics::PLAYER_MAX_VEL,
                            config::physics::PLAYER_FRICTION_COEFF,
                        ),
                    )
                    .insert(physics::Collider::solid_from_shapes(
                        svg_data.groups[svgdata::COLLISION].clone(),
                    ))
                    .id()
            }
            Prefab::Princess => {
                let svg_data = svg_datas
                    .get(
                        &svg_data_handles.handles
                            [&asset::ObjectLabel::Princess],
                    )
                    .unwrap();
                commands
                    .spawn_bundle(SpriteBundle {
                        material: material_handles.handles
                            [&asset::ObjectLabel::Princess]
                            .clone(),
                        transform: Transform::from_translation(Vec3::from((
                            spawn.position,
                            depths::PRINCESS,
                        )))
                        .scaled(sizes::PRINCESS / svg_data.size),
                        sprite: Sprite::new(svg_data.size),
                        ..Default::default()
                    })
                    .insert(physics::Collider::solid_from_shapes(
                        svg_data.groups[svgdata::COLLISION].clone(),
                    ))
                    .id()
                //.insert(Princess or smth)
            }
            Prefab::WorldMap(world_map_prefab) => {
                let (material, svg_data) = match world_map_prefab.map_id {
                    0 => (
                        &material_handles.handles
                            [&asset::ObjectLabel::WorldMap1],
                        svg_datas
                            .get(
                                &svg_data_handles.handles
                                    [&asset::ObjectLabel::WorldMap1],
                            )
                            .unwrap(),
                    ),
                    _ => unreachable!(),
                };
                commands
                    .spawn_bundle(SpriteBundle {
                        material: material.clone(),
                        transform: Transform::from_translation(Vec3::from((
                            spawn.position,
                            depths::WORLD_MAP,
                        )))
                        .scaled(sizes::WORLD_MAP1 / svg_data.size),
                        sprite: Sprite::new(svg_data.size),
                        ..Default::default()
                    })
                    .id()
            }
        };

        overlord_new_children.push(new_child);
    }

    commands
        .entity(overlord_query.single().unwrap())
        .push_children(&overlord_new_children);
}
