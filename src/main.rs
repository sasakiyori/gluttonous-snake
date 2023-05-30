mod bean;
mod snake;

use bevy::{prelude::*, window::PrimaryWindow};

use bean::BeanPlugin;
use snake::SnakePlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::WHITE))
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        .add_plugin(SnakePlugin)
        .add_plugin(BeanPlugin)
        .run();
}

fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}
