use bevy::prelude::*;

use super::components::*;
use super::resources::MenuResources;
use super::styles::*;

use crate::score::resources::{HighestScore, Score};
use crate::util::resources::*;

pub fn draw_main_menu(mut commands: Commands, menu_resource: Res<MenuResources>) {
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
                                text_style(&menu_resource, 64., Color::BLACK),
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
                                text_style(&menu_resource, 32., Color::BLACK),
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

pub fn draw_game_over_menu(
    mut commands: Commands,
    score: Res<Score>,
    highest_score: Res<HighestScore>,
    menu_resource: Res<MenuResources>,
) {
    commands
        .spawn(
            // game over menu box
            (
                NodeBundle {
                    style: GAME_OVER_MENU_STYLE,
                    ..default()
                },
                GameOverMenu,
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
                                "Game Over",
                                text_style(&menu_resource, 64., Color::BLACK),
                            ));
                        },
                    );
            },
        )
        .with_children(
            // highest score box
            |parent| {
                parent.spawn(TextBundle::from_section(
                    format!("History Highest Score: {}", highest_score.0),
                    text_style(&menu_resource, 32., Color::BLACK),
                ));
            },
        )
        .with_children(
            // score box
            |parent| {
                parent.spawn(TextBundle::from_section(
                    format!("Score: {}", score.0),
                    text_style(&menu_resource, 32., Color::BLACK),
                ));
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
                                "Play Again",
                                text_style(&menu_resource, 32., Color::BLACK),
                            ));
                        },
                    );
            },
        );
}

pub fn despawn_game_over_menu(
    mut commands: Commands,
    menu_query: Query<Entity, With<GameOverMenu>>,
) {
    if let Ok(entity) = menu_query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn interact_with_play_button(
    mut button_query: Query<&Interaction, With<PlayButton>>,
    mut score: ResMut<Score>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in button_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                next_state.set(GameState::Playing);
                score.0 = 0;
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
