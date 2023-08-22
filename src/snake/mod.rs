pub mod components;
pub mod cst;
mod resources;
mod systems;

use bevy::prelude::*;

use resources::*;
use systems::*;

use crate::util::resources::GameState;
use bevy::ecs::prelude::IntoSystemSetConfig;
use bevy::time::common_conditions::on_timer;
use std::time::Duration;
pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SnakeMoveTimer>()
            .insert_resource(FixedTime::new_from_secs(1.0 / 20.0))
            .add_startup_system(cache_snake_resources)
            .add_system(spawn_snake.in_schedule(OnEnter(GameState::Playing)))
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
                    .in_set(OnUpdate(GameState::Playing))
                    .chain()
                    .in_schedule(CoreSchedule::FixedUpdate),
            );
        // let timer = Duration::from_secs_f32(0.5);
        // app.init_resource::<SnakeMoveTimer>()
        //     // .insert_resource(FixedTime::new_from_secs(1.0 / 30.0))
        //     .add_startup_system(cache_snake_resources)
        //     .add_system(spawn_snake.in_schedule(OnEnter(GameState::Playing)))
        //     // 1. snake_eat_bean_check must run before snake_move,
        //     // but I am not so sure that snake_dead_check should do that as well.
        //     // 2. since there has entity spawned in the snake_eat_bean_check,
        //     // I think we should run apply_system_buffers to flush the Query for Snake
        //     .add_systems(
        //         (
        //             snake_dead_check.run_if(on_timer(timer)),
        //             snake_eat_bean_check.run_if(on_timer(timer)),
        //             apply_system_buffers.run_if(on_timer(timer)),
        //             snake_move.run_if(on_timer(timer)),
        //         )
        //             .in_set(OnUpdate(GameState::Playing))
        //             .chain(), // .in_schedule(CoreSchedule::FixedUpdate),
        //     );
    }
}
