use bevy::prelude::*;

use crate::game::physics::Collider;
use crate::game::player::Player;

pub fn test_system(player_query: Query<&Collider, With<Player>>) {
    if let Ok(collider) = player_query.single() {
        let recent_collisions = collider.get_recent_collisions();
        if recent_collisions.len() == 0 {
            println!("Not colliding");
        } else {
            println!("Colliding with {:?}", recent_collisions[0].entity);
        }
    }
}
