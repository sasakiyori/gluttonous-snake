use bevy::prelude::*;

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
pub enum GameState {
    #[default]
    Menu,
    Playing,
}

#[derive(Resource)]
pub struct GameOver;
