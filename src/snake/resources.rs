use bevy::prelude::*;

pub const SNAKE_SIZE: f32 = 18.0;
pub const SNAKE_RADIUS: f32 = SNAKE_SIZE / 2.0;
pub const SNAKE_SPEED: f32 = SNAKE_SIZE / 6.0;
pub const SNAKE_MOVE_DURATION: f32 = 0.1;

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
}

impl Default for SnakeMoveTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(SNAKE_MOVE_DURATION, TimerMode::Repeating),
        }
    }
}
