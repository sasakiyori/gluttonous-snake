mod components;
mod systems;

use bevy::prelude::*;
use systems::*;

pub struct BeanPlugin;

impl Plugin for BeanPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_bean);
    }
}
