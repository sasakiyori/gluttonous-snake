use super::{
    components::{Direction, Snake, SnakePiece},
    resources::SnakeResources,
};

use std::collections::LinkedList;

use crate::bean::components::Bean;

use bevy::{prelude::*, window::PrimaryWindow};

const SNAKE_SIZE: f32 = 18.0;
const SNAKE_SPEED: f32 = SNAKE_SIZE * 5.0;

pub fn spawn_snake(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    resource_query: Res<SnakeResources>,
) {
    let window: &Window = window_query.get_single().unwrap();
    let mut snake = LinkedList::new();
    snake.push_back(SnakePiece(Direction::None));
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: resource_query.image.clone(),
            ..default()
        },
        Snake { body: snake },
    ));
}

pub fn snake_direction(keyboard_input: Res<Input<KeyCode>>, mut snake_query: Query<&mut Snake>) {
    let mut direction = Direction::None;
    if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
        direction = Direction::Left;
    }
    if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
        direction = Direction::Right;
    }
    if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
        direction = Direction::Up;
    }
    if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
        direction = Direction::Down;
    }
    if direction == Direction::None {
        return;
    }

    if let Ok(mut snake) = snake_query.get_single_mut() {
        let mut iter = snake.body.iter_mut();
        let mut prev = Direction::None;
        loop {
            match iter.next() {
                Some(piece) => {
                    let tmp = piece.0;
                    piece.0 = prev;
                    prev = tmp;
                }
                None => break,
            }
        }
        if let Some(head) = snake.body.front_mut() {
            head.0 = direction;
        }
    }
}

pub fn snake_movement(
    snake_query: Query<&Snake>,
    mut snake_transform_query: Query<&mut Transform, With<Snake>>,
    time: Res<Time>,
) {
    if let Ok(snake) = snake_query.get_single() {
        for piece in snake.body.iter() {
            if piece.0 == Direction::None {
                continue;
            }
            if let Ok(mut transform) = snake_transform_query.get_single_mut() {
                transform.translation += match piece.0 {
                    Direction::Left => Vec3::new(-SNAKE_SPEED * time.delta_seconds(), 0.0, 0.0),
                    Direction::Right => Vec3::new(SNAKE_SPEED * time.delta_seconds(), 0.0, 0.0),
                    Direction::Up => Vec3::new(0.0, SNAKE_SPEED * time.delta_seconds(), 0.0),
                    Direction::Down => Vec3::new(0.0, -SNAKE_SPEED * time.delta_seconds(), 0.0),
                    _ => Vec3::ZERO,
                }
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
