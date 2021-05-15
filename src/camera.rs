use bevy::ecs::system::QuerySingleError;
use bevy::prelude::*;

use crate::game::player::Player;

/// Marker component for the Main Menu Ui Camera
pub struct UiCamera;

/// Marker component for the Game Camera
pub struct GameCamera;

/* Systems*/

/// Startup system, runs exactly once and spawns all cameras
pub fn spawn(mut commands: Commands) {
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(UiCamera);
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(GameCamera);
}

type PlayerQuery<'a, 'b> = Query<'a, &'b Transform, With<Player>>;
type CameraQuery<'a, 'b> = Query<'a, &'b mut Transform, With<GameCamera>>;
pub fn movement(mut q: QuerySet<(PlayerQuery, CameraQuery)>) {
    let player_query = q.q0();

    // TODO: add logging here
    let player_transform = match player_query.single() {
        Ok(transform) => transform,
        Err(QuerySingleError::NoEntities(_)) => return,
        Err(QuerySingleError::MultipleEntities(e)) => {
            panic!("Mutliple player entities: {}", e);
        }
    };
    let player_position = Vec2::new(
        player_transform.translation.x,
        player_transform.translation.y,
    );

    let camera_query = q.q1_mut();
    let mut camera_transform = match camera_query.single_mut() {
        Ok(transform) => transform,
        Err(QuerySingleError::NoEntities(_)) => return,
        Err(QuerySingleError::MultipleEntities(e)) => {
            panic!("Mutliple player entities: {}", e);
        }
    };

    camera_transform.translation.x = player_position.x;
    camera_transform.translation.y = player_position.y;
}
