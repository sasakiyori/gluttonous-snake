mod bean;
mod menu;
mod score;
mod snake;
mod util;

use bevy::{prelude::*, window::PrimaryWindow};

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

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}
