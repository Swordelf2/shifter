//! Menu state describes main menu of the app

use bevy::prelude::*;

use crate::asset::FontHandles;
use crate::state::AppState;

/// Marker component for the main menu UiCamera
pub struct UiCamera;

/// Component, indicating that this is one of the main menu buttons,
/// choosing which map to load
#[derive(Clone, Copy)]
pub struct MapButton {
    pub map_idx: usize,
}

/// Marker component for Main Menu
pub struct Menu;

/* Systems */

/// Runs on entering Menu state
pub fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    font_handles: Res<FontHandles>,
) {
    // Ui camera
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(UiCamera);

    // Root of the menu entities hierarchy
    commands
        .spawn_bundle(NodeBundle::default())
        .insert(Menu)
        .with_children(|parent| {
            // First map button
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        // center button
                        margin: Rect::all(Val::Auto),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    // TODO this should be in impl FromWorld for a resource
                    material: materials
                        .add(ColorMaterial::from(Color::rgb(0.95, 0.95, 0.1))),
                    ..Default::default()
                })
                .insert(MapButton { map_idx: 0 })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Map1",
                            TextStyle {
                                font: font_handles.noto_sans_regular.clone(),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    });
                });
            // Second map button
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        // center button
                        margin: Rect::all(Val::Auto),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    // TODO this should be in impl FromWorld for a resource
                    material: materials
                        .add(ColorMaterial::from(Color::rgb(0.95, 0.95, 0.1))),
                    ..Default::default()
                })
                .insert(MapButton { map_idx: 1 })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Map2",
                            TextStyle {
                                font: font_handles.noto_sans_regular.clone(),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    });
                });
        });
}

/// Watch for button changes, and transition to next state once it's clicked
pub fn update(
    mut commands: Commands,
    mut state: ResMut<State<AppState>>,
    query: Query<(&Interaction, &MapButton), Changed<Interaction>>,
) {
    for (interaction, map_button) in query.iter() {
        match *interaction {
            Interaction::Clicked => {
                commands.insert_resource(*map_button);
                state.set(AppState::Game).unwrap();
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

/// Cleanup upon exiting the state
pub fn exit(
    mut commands: Commands,
    menu_query: Query<Entity, With<Menu>>,
    camera_query: Query<Entity, With<UiCamera>>,
) {
    // Despawn the whole menu tree
    commands
        .entity(menu_query.single().unwrap())
        .despawn_recursive();
    // Despawn the camera
    commands
        .entity(camera_query.single().unwrap())
        .despawn_recursive();
}
