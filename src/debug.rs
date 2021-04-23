use bevy::prelude::*;

pub fn test_system(
    player_query: Query<&Transform, With<crate::game::player::Player>>,
) {
    for transform in player_query.iter() {
        dbg!(transform.rotation.to_axis_angle());
    }
}
// TODO
pub fn print_all_cameras() {}

// TODO
pub fn print_all_entities() {}
