use bevy::prelude::*;

use crate::asset;
use crate::asset::{
    svgdata, MaterialHandles, ObjectLabel, SvgData, SvgDataHandles,
};
use crate::config;
use crate::config::{depths, sizes};
use crate::util::TransformExt;

use super::physics;
use super::physics::{Collider, DynamicObject};
use super::player;

// Resource, stat stores all prefabs
pub struct Prefabs {
    pub player: Player,
    pub princess: Princess,
    pub world_map1: WorldMap1,
}

#[derive(Bundle, Clone)]
pub struct Player {
    #[bundle]
    pub sprite_bundle: SpriteBundle,

    pub name: Name,
    pub player: player::Player,
    pub dynamic_object: DynamicObject,
    pub collider: Collider,
}

#[derive(Bundle, Clone)]
pub struct Princess {
    #[bundle]
    pub sprite_bundle: SpriteBundle,

    pub name: Name,
    pub collider: Collider,
}

#[derive(Bundle, Clone)]
pub struct WorldMap1 {
    #[bundle]
    pub sprite_bundle: SpriteBundle,

    pub name: Name,
}

// Initialize Prefabs
pub fn initialize_prefabs(
    material_handles: &MaterialHandles,
    svg_datas: &Assets<SvgData>,
    svg_data_handles: &SvgDataHandles,
) -> Prefabs {
    let player = {
        let name = "Player";
        let object_label = asset::ObjectLabel::Player;
        let size = sizes::PLAYER;
        let depth = depths::PLAYER;
        let max_vel = config::physics::PLAYER_MAX_VEL;
        let friction_coeff = config::physics::PLAYER_FRICTION_COEFF;
        let svg_data = svg_datas
            .get(&svg_data_handles.handles[&object_label])
            .unwrap();
        Player {
            sprite_bundle: SpriteBundle {
                material: material_handles.handles[&object_label].clone(),
                transform: Transform::from_xyz(0.0, 0.0, depth)
                    .scaled(size / svg_data.size),
                sprite: Sprite::new(svg_data.size),
                ..Default::default()
            },
            name: Name::new(name),
            player: player::Player,
            dynamic_object:
                physics::DynamicObject::from_max_vel_and_friction_coeff(
                    max_vel,
                    friction_coeff,
                ),
            collider: physics::Collider::solid_from_shapes(
                svg_data.groups[svgdata::COLLISION].clone(),
            ),
        }
    };

    let princess = {
        let name = "Princess";
        let object_label = asset::ObjectLabel::Princess;
        let size = sizes::PRINCESS;
        let depth = depths::PRINCESS;
        let svg_data = svg_datas
            .get(&svg_data_handles.handles[&object_label])
            .unwrap();
        Princess {
            sprite_bundle: SpriteBundle {
                material: material_handles.handles[&object_label].clone(),
                transform: Transform::from_xyz(0.0, 0.0, depth)
                    .scaled(size / svg_data.size),
                sprite: Sprite::new(svg_data.size),
                ..Default::default()
            },
            name: Name::new(name),
            collider: physics::Collider::solid_from_shapes(
                svg_data.groups[svgdata::COLLISION].clone(),
            ),
        }
    };

    let world_map1 = {
        let name = "WorldMap1";
        let object_label = asset::ObjectLabel::WorldMap1;
        let size = sizes::WORLD_MAP1;
        let depth = depths::WORLD_MAP;
        let svg_data = svg_datas
            .get(&svg_data_handles.handles[&object_label])
            .unwrap();
        WorldMap1 {
            sprite_bundle: SpriteBundle {
                material: material_handles.handles[&object_label].clone(),
                transform: Transform::from_xyz(0.0, 0.0, depth)
                    .scaled(size / svg_data.size),
                sprite: Sprite::new(svg_data.size),
                ..Default::default()
            },
            name: Name::new(name),
        }
    };

    Prefabs {
        player,
        princess,
        world_map1,
    }
}
