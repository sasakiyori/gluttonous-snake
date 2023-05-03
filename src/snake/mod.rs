pub mod components;
mod resources;
mod systems;

use bevy::prelude::*;

use resources::cache_snake_resources;
use systems::*;

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(cache_snake_resources)
            // usually startup systems are in the same stage
            // so if we don't use the base set or create a separate startup stage, the cached resources
            //    will not be found because they are not loaded yet.
            // ref:
            // https://github.com/bevyengine/bevy/discussions/8335
            // https://www.reddit.com/r/bevy/comments/xv1hog/startup_systems_not_working_as_expected/
            .add_startup_system(spawn_snake.in_base_set(StartupSet::PostStartup))
            .add_system(snake_direction)
            .add_system(snake_movement)
            .add_system(snake_dead_check)
            .add_system(snake_eat_bean_check);
    }
}
