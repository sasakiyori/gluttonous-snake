use bevy::prelude::*;

pub const SNAKE_SIZE: f32 = 18.0;
pub const SNAKE_SPEED: f32 = SNAKE_SIZE / 6.0;
pub const SNAKE_MOVE_DURATION: f32 = 0.1;

#[derive(Resource)]
pub struct SnakeResources {
    pub image: Handle<Image>,
}

pub fn cache_snake_resources(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(SnakeResources {
        image: asset_server.load("snake.png"),
    });
}

#[derive(Resource)]
pub struct SnakeMoveTimer {
    pub timer: Timer,
}

impl Default for SnakeMoveTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(SNAKE_MOVE_DURATION, TimerMode::Repeating),
        }
    }
}
