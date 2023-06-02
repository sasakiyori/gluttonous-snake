use bevy::prelude::*;

use super::components::*;
use super::styles::*;

use crate::util::resources::*;

pub fn draw_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(
            // main menu box
            (
                NodeBundle {
                    style: MAIN_MENU_STYLE,
                    ..default()
                },
                MainMenu,
            ),
        )
        .with_children(
            // title box
            |parent| {
                parent
                    .spawn(NodeBundle {
                        style: TITLE_STYLE,
                        ..default()
                    })
                    .with_children(
                        // title text
                        |parent| {
                            parent.spawn(TextBundle::from_section(
                                "Gluttonous Snake",
                                TextStyle {
                                    font: asset_server.load("font/orange juice 2.0.ttf"),
                                    font_size: 64.,
                                    color: Color::BLACK,
                                },
                            ));
                        },
                    );
            },
        )
        .with_children(
            // play button box
            |parent| {
                parent
                    .spawn((
                        ButtonBundle {
                            style: PLAY_BUTTON_STYLE,
                            ..default()
                        },
                        PlayButton,
                    ))
                    .with_children(
                        // play button text
                        |parent| {
                            parent.spawn(TextBundle::from_section(
                                "Play",
                                TextStyle {
                                    font: asset_server.load("font/orange juice 2.0.ttf"),
                                    font_size: 32.,
                                    color: Color::BLACK,
                                },
                            ));
                        },
                    );
            },
        );
}

pub fn despawn_main_menu(mut commands: Commands, menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(entity) = menu_query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn interact_with_play_button(
    mut button_query: Query<&Interaction, With<PlayButton>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in button_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                next_state.set(GameState::Playing);
            }
            _ => {}
        }
    }
}

pub fn handle_game_over(
    mut event_reader: EventReader<GameOver>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for _ in event_reader.iter() {
        next_state.set(GameState::GameOver);
    }
}
