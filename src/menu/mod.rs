use bevy::prelude::*;

mod components;
mod styles;
mod systems;

use systems::*;

use crate::util::resources::GameState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(draw_main_menu.in_schedule(OnEnter(GameState::Menu)))
            .add_system(despawn_main_menu.in_schedule(OnExit(GameState::Menu)))
            .add_system(interact_with_play_button)
            .add_system(handle_game_over);
    }
}
