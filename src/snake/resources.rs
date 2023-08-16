use bevy::prelude::*;

use super::cst::SNAKE_MOVE_DURATION;

#[derive(Resource)]
pub struct SnakeResources {
    pub image: Handle<Image>,
    pub eat: Handle<AudioSource>,
    pub die: Handle<AudioSource>,
}

pub fn cache_snake_resources(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(SnakeResources {
        image: asset_server.load("sprite/snake.png"),
        eat: asset_server.load("audio/impactWood_medium_000.ogg"),
        die: asset_server.load("audio/footstep_snow_000.ogg"),
    });
}

#[derive(Resource)]
pub struct SnakeMoveTimer {
    pub timer: Timer,
    pub status: i32,
}

impl Default for SnakeMoveTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(SNAKE_MOVE_DURATION, TimerMode::Repeating),
            status: 1,
        }
    }
}
