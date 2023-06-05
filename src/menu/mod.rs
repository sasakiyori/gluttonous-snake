use bevy::prelude::*;

mod components;
mod resources;
mod styles;
mod systems;

use resources::cache_menu_resources;
use systems::*;

use crate::util::resources::GameState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(cache_menu_resources)
            .add_system(draw_main_menu.in_schedule(OnEnter(GameState::Menu)))
            .add_system(despawn_main_menu.in_schedule(OnExit(GameState::Menu)))
            .add_system(draw_game_over_menu.in_schedule(OnEnter(GameState::GameOver)))
            .add_system(despawn_game_over_menu.in_schedule(OnExit(GameState::GameOver)))
            .add_system(interact_with_play_button)
            .add_system(handle_game_over);
    }
}
