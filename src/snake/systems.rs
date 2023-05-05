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
    let mut snake_body = LinkedList::new();
    let piece = commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
                texture: resource_query.image.clone(),
                ..default()
            },
            SnakePiece,
        ))
        .id();
    snake_body.push_back(piece);
    commands.spawn(Snake {
        body: snake_body,
        direction: Direction::None,
    });
}

pub fn snake_direction(keyboard_input: Res<Input<KeyCode>>, mut snake_query: Query<&mut Snake>) {
    if let Ok(mut snake) = snake_query.get_single_mut() {
        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            snake.direction = Direction::Left;
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            snake.direction = Direction::Right;
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            snake.direction = Direction::Up;
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            snake.direction = Direction::Down;
        }
    }
}

pub fn snake_movement(
    mut snake_query: Query<&mut Snake>,
    mut snake_transform_query: Query<(Entity, &mut Transform), With<SnakePiece>>,
    time: Res<Time>,
) {
    if let Ok(mut snake) = snake_query.get_single_mut() {
        if let Some(tail) = snake.body.pop_back() {
            let head = match snake.body.front() {
                Some(piece) => *piece,
                None => tail,
            };
            snake.body.push_front(tail);
            // if there is only one snake piece
            if head == tail {
                if let Ok((_, mut transform)) = snake_transform_query.get_single_mut() {
                    transform.translation += match snake.direction {
                        Direction::Left => Vec3::new(-SNAKE_SPEED * time.delta_seconds(), 0.0, 0.0),
                        Direction::Right => Vec3::new(SNAKE_SPEED * time.delta_seconds(), 0.0, 0.0),
                        Direction::Up => Vec3::new(0.0, SNAKE_SPEED * time.delta_seconds(), 0.0),
                        Direction::Down => Vec3::new(0.0, -SNAKE_SPEED * time.delta_seconds(), 0.0),
                        _ => Vec3::ZERO,
                    }
                }
            } else {
                let mut head_translation = Vec3::new(0.0, 0.0, 0.0);
                if let Ok((_, head_transform)) = snake_transform_query.get(head) {
                    head_translation = head_transform.translation;
                }
                if let Ok((_, mut tail_transform)) = snake_transform_query.get_mut(tail) {
                    tail_transform.translation = head_translation
                        + match snake.direction {
                            Direction::Left => {
                                Vec3::new(-SNAKE_SPEED * time.delta_seconds(), 0.0, 0.0)
                            }
                            Direction::Right => {
                                Vec3::new(SNAKE_SPEED * time.delta_seconds(), 0.0, 0.0)
                            }
                            Direction::Up => {
                                Vec3::new(0.0, SNAKE_SPEED * time.delta_seconds(), 0.0)
                            }
                            Direction::Down => {
                                Vec3::new(0.0, -SNAKE_SPEED * time.delta_seconds(), 0.0)
                            }
                            _ => Vec3::ZERO,
                        };
                }
            }
        }
    }
}

pub fn snake_dead_check(
    mut commands: Commands,
    mut snake_query: Query<&mut Snake>,
    snake_transform_query: Query<(Entity, &Transform), With<SnakePiece>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // snake will never be destroyed
    let mut snake = snake_query.get_single_mut().unwrap();

    let snake_head_entity = match snake.body.front() {
        Some(entity) => entity,
        None => return,
    };
    let (_, snake_head_transform) = snake_transform_query.get(*snake_head_entity).unwrap();

    let window = window_query.get_single().unwrap();
    let x_min = SNAKE_SIZE / 2.0;
    let x_max = window.width() - x_min;
    let y_min = SNAKE_SIZE / 2.0;
    let y_max = window.height() - y_min;

    let translation = snake_head_transform.translation;
    if translation.x < x_min
        || translation.x > x_max
        || translation.y < y_min
        || translation.y > y_max
    {
        println!("snake dead");
        // clean snake piece entity list
        snake.body.clear();
        // destroy snake pieces
        for iter in snake_transform_query.iter() {
            commands.entity(iter.0).despawn();
        }
    }
}

pub fn snake_eat_bean_check(
    mut commands: Commands,
    snake_query: Query<&Snake>,
    snake_transform_query: Query<(Entity, &Transform), With<SnakePiece>>,
    bean_query: Query<(Entity, &Transform), With<Bean>>,
) {
    // snake will never be destroyed
    let snake = snake_query.get_single().unwrap();

    let snake_head_entity = match snake.body.front() {
        Some(entity) => entity,
        None => return,
    };
    let (_, snake_head_transform) = snake_transform_query.get(*snake_head_entity).unwrap();

    if let Ok((bean_entity, bean_transform)) = bean_query.get_single() {
        let distance = snake_head_transform
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
