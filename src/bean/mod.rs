pub mod components;
pub mod cst;
mod resources;
mod systems;

use bevy::prelude::{App, Plugin};
use resources::cache_bean_resources;
use systems::spawn_bean;

pub struct BeanPlugin;

impl Plugin for BeanPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(cache_bean_resources)
            .add_system(spawn_bean);
    }
}
