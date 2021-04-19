//! Menu state describes main menu of the app

use bevy::prelude::*;

use crate::asset::FontHandles;
use crate::state::AppState;

/// Component, indicating that this is one of the main menu buttons,
/// choosing which map to load
pub struct MapButton;

/// Market component for Menu UI camera
pub struct UiCamera;

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

    // First map button
    commands.spawn_bundle(UiCameraBundle::default());
    commands
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
        .insert(MapButton)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Button",
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
}

pub fn update(
    mut state: ResMut<State<AppState>>,
    query: Query<(&Interaction, &MapButton), Changed<Interaction>>,
) {
    for (interaction, _) in query.iter() {
        match *interaction {
            Interaction::Clicked => {
                state.set(AppState::Game).unwrap();
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

pub fn exit(mut commands: Commands, query: Query<Entity, With<UiCamera>>) {
    println!("Despawning!");
    commands.entity(query.single().unwrap()).despawn();
}
