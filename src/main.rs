mod bean;
mod menu;
mod score;
mod snake;
mod util;

use bevy::ecs::prelude::IntoSystemSetConfig;
use bevy::time::common_conditions::on_timer;
use bevy::{prelude::*, window::PrimaryWindow};
use std::time::Duration;
use util::resources::GameState;

use bean::BeanPlugin;
use menu::MenuPlugin;
use score::ScorePlugin;
use snake::SnakePlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::WHITE))
        .add_plugins(DefaultPlugins)
        .add_state::<GameState>()
        .add_startup_system(spawn_camera)
        .add_plugin(MenuPlugin)
        .add_plugin(SnakePlugin)
        .add_plugin(BeanPlugin)
        .add_plugin(ScorePlugin)
        .run();
}

fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    // dbg!(window.width(), window.height()); 1280.0 720.0
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}
