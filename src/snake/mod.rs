pub mod components;
pub mod resources;
mod systems;

use bevy::prelude::*;

use resources::*;
use systems::*;

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SnakeMoveTimer>()
            .add_startup_system(cache_snake_resources)
            // usually startup systems are in the same stage
            // so if we don't use the base set or create a separate startup stage, the cached resources
            //    will not be found because they are not loaded yet.
            // ref:
            // https://github.com/bevyengine/bevy/discussions/8335
            // https://www.reddit.com/r/bevy/comments/xv1hog/startup_systems_not_working_as_expected/
            .add_startup_system(spawn_snake.in_base_set(StartupSet::PostStartup))
            // 1. snake_eat_bean_check must run before snake_move,
            // but I am not so sure that snake_dead_check should do that as well.
            // 2. since there has entity spawned in the snake_eat_bean_check,
            // I think we should run apply_system_buffers to flush the Query for Snake
            .add_systems(
                (
                    snake_dead_check,
                    snake_eat_bean_check,
                    apply_system_buffers,
                    snake_move,
                )
                    .chain(),
            );
    }
}
