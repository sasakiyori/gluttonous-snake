pub mod components;
mod systems;

use bevy::prelude::{App, Plugin};
use systems::*;

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_snake)
            .add_system(snake_direction)
            .add_system(snake_movement)
            .add_system(snake_dead_check);
    }
}
