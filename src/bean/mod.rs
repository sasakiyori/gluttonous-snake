pub mod components;
pub mod cst;
mod resources;
mod systems;

use bevy::prelude::*;
use resources::cache_bean_resources;
use systems::spawn_bean;

use crate::util::resources::GameState;

pub struct BeanPlugin;

impl Plugin for BeanPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(cache_bean_resources)
            .add_system(spawn_bean.run_if(in_state(GameState::Playing)));
    }
}
