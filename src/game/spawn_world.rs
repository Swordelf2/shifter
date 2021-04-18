use bevy::prelude::*;

use crate::asset;
use crate::config::{depths, map_colors, CELL_SIZE};

use super::player::Player;

pub fn spawn_world(
    mut commands: Commands,
    material_handles: Res<asset::MaterialHandles>,
    map_img: Res<asset::MapImage>,
) {
    /*
    // Load player texture atlas
    let (player_atlas_len, player_atlas_handle) = {
        let texture_handle = asset_server.load("textures/player/player.png");
        let atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(720.0, 490.0), 18, 1);
        // let something_handle = asset_server.get_handle("path to something within the atlas");
        // let something_index = atlas.get_texture_index(&something_handle).unwrap();
        (atlas.textures.len(), atlas_assets.add(atlas))
    };
    */

    // Iterate over the pixels of the map image and spawn corresponding cells (player and hazards)
    let map_img = &map_img.0;
    for (x, mut y, &pixel) in map_img.enumerate_pixels() {
        // Invert the y axis
        y = map_img.height() - y - 1;
        // Calculate cell position
        let cell_pos_x = x as f32 * CELL_SIZE + CELL_SIZE / 2.0;
        let cell_pos_y = y as f32 * CELL_SIZE + CELL_SIZE / 2.0;

        match pixel.0 {
            // Red = Princess (finish cell)
            map_colors::PRINCESS => {
                // Spawn princess
                commands.spawn_bundle(SpriteBundle {
                    material: material_handles.princess.clone(),
                    transform: Transform::from_translation(Vec3::new(
                        cell_pos_x,
                        cell_pos_y,
                        depths::PRINCESS,
                    )),
                    sprite: Sprite::new(Vec2::new(CELL_SIZE, CELL_SIZE)),
                    ..Default::default()
                });
                //.insert(Collider::Princess);
            }
            // Green = Start cell
            map_colors::PLAYER => {
                // Spawn player
                commands
                    .spawn_bundle(SpriteBundle {
                        material: material_handles.player.clone(),
                        transform: Transform::from_translation(Vec3::new(
                            cell_pos_x,
                            cell_pos_y,
                            depths::PLAYER,
                        )),
                        sprite: Sprite::new(Vec2::new(
                            2.0 * CELL_SIZE,
                            2.0 * CELL_SIZE,
                        )),
                        ..Default::default()
                    })
                    .insert(Player)
                    // Spawn child camera
                    .with_children(|parent| {
                        let mut camera = OrthographicCameraBundle::new_2d();
                        // TODO camera bug: hazard are drawn with this line ??
                        // absolute z should be = `far` - `eps`
                        camera.transform.translation.z =
                            1000.0 - 0.5 - depths::PLAYER;
                        parent.spawn_bundle(camera);
                    });
                /*
                // Store the start position as a resource
                commands.insert_resource(StartPos {
                    x: cell_pos_x,
                    y: cell_pos_y,
                });
                */
            }
            // Blue = Hazard cell
            map_colors::HAZARD => {
                // Spawn a hazard cell
                commands.spawn_bundle(SpriteBundle {
                    material: material_handles.hazard.clone(),
                    transform: Transform::from_translation(Vec3::new(
                        cell_pos_x,
                        cell_pos_y,
                        depths::HAZARD,
                    )),
                    sprite: Sprite::new(Vec2::new(CELL_SIZE, CELL_SIZE)),
                    ..Default::default()
                });
                //.insert(Collider::Hazard);
            }
            _ => {}
        }
    }
}
