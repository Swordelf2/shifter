use bevy::prelude::*;

use crate::config::{map, CELL_SIZE};
use crate::{asset, menu};

use super::spawner::Spawn;

pub fn spawn(
    map_images: Res<asset::MapImages>,
    map_button: Res<menu::MapButton>,
    mut spawns: EventWriter<Spawn>,
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

    // Iterate over the pixels of the map image and spawn corresponding entities
    // by sending `Spawn` event to the `spawn` system
    let map_image = &map_images.images[map_button.map_idx];
    for (x, mut y, &pixel) in map_image.enumerate_pixels() {
        // Invert the y axis
        y = map_image.height() - y - 1;
        // Calculate cell position
        let cell_pos_x = x as f32 * CELL_SIZE + CELL_SIZE / 2.0;
        let cell_pos_y = y as f32 * CELL_SIZE + CELL_SIZE / 2.0;

        if let Some(prefab) = map::pixel2prefab(pixel.0) {
            spawns.send(Spawn {
                position: Vec2::new(cell_pos_x, cell_pos_y),
                prefab,
            });
        }
    }
}
