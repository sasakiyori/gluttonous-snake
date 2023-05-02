use super::components::{Direction, Snake};

use crate::bean::components::Bean;

use bevy::{prelude::*, window::PrimaryWindow};

const SNAKE_SIZE: f32 = 18.0;
const SNAKE_SPEED: f32 = SNAKE_SIZE * 5.0;

pub fn spawn_snake(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("snake.png"),
            ..default()
        },
        Snake(Direction::None),
    ));
}

pub fn snake_direction(keyboard_input: Res<Input<KeyCode>>, mut snake_query: Query<&mut Snake>) {
    if let Ok(mut snake) = snake_query.get_single_mut() {
        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            snake.0 = Direction::Left;
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            snake.0 = Direction::Right;
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            snake.0 = Direction::Up;
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            snake.0 = Direction::Down;
        }
    }
}

pub fn snake_movement(
    snake_query: Query<&Snake>,
    mut snake_transform_query: Query<&mut Transform, With<Snake>>,
    time: Res<Time>,
) {
    if let Ok(snake) = snake_query.get_single() {
        if snake.0 == Direction::None {
            return;
        }
        if let Ok(mut transform) = snake_transform_query.get_single_mut() {
            transform.translation += match snake.0 {
                Direction::Left => Vec3::new(-SNAKE_SPEED * time.delta_seconds(), 0.0, 0.0),
                Direction::Right => Vec3::new(SNAKE_SPEED * time.delta_seconds(), 0.0, 0.0),
                Direction::Up => Vec3::new(0.0, SNAKE_SPEED * time.delta_seconds(), 0.0),
                Direction::Down => Vec3::new(0.0, -SNAKE_SPEED * time.delta_seconds(), 0.0),
                _ => Vec3::ZERO,
            }
        }
    }
}

pub fn snake_dead_check(
    mut commands: Commands,
    snake_query: Query<(Entity, &Transform), With<Snake>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok((snake_entity, snake_transform)) = snake_query.get_single() {
        let window = window_query.get_single().unwrap();
        let x_min = SNAKE_SIZE / 2.0;
        let x_max = window.width() - x_min;
        let y_min = SNAKE_SIZE / 2.0;
        let y_max = window.height() - y_min;

        let translation = snake_transform.translation;
        if translation.x < x_min
            || translation.x > x_max
            || translation.y < y_min
            || translation.y > y_max
        {
            println!("snake dead");
            commands.entity(snake_entity).despawn();
        }
    }
}

pub fn snake_eat_bean_check(
    mut commands: Commands,
    snake_query: Query<&Transform, With<Snake>>,
    bean_query: Query<(Entity, &Transform), With<Bean>>,
) {
    if let Ok(snake_transform) = snake_query.get_single() {
        if let Ok((bean_entity, bean_transform)) = bean_query.get_single() {
            let distance = snake_transform
                .translation
                .distance(bean_transform.translation);
            let snake_radius = SNAKE_SIZE / 2.0;
            // TODO: use unique size
            let bean_radius = SNAKE_SIZE / 2.0;
            if distance < snake_radius + bean_radius {
                println!("snake eat bean!");
                commands.entity(bean_entity).despawn();
            }
        }
    }
}
