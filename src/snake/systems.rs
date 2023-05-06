use super::{
    components::{Direction, Snake},
    resources::{SnakeMoveTimer, SnakeResources},
};

use crate::bean::components::Bean;

use bevy::{prelude::*, window::PrimaryWindow};

const SNAKE_SIZE: f32 = 18.0;
const SNAKE_SPEED: f32 = SNAKE_SIZE / 6.0;

pub fn spawn_snake(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    resource_query: Res<SnakeResources>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: resource_query.image.clone(),
            ..default()
        },
        Snake(Direction::None),
    ));
}

pub fn snake_direction(
    keyboard_input: Res<Input<KeyCode>>,
    mut snake_query: Query<&mut Snake>,
    mut snake_move_timer: ResMut<SnakeMoveTimer>,
    time: Res<Time>,
) {
    if !snake_move_timer.timer.tick(time.delta()).just_finished() {
        return;
    }

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

    for mut iter in snake_query.iter_mut() {
        let tmp = iter.0;
        iter.0 = direction;
        direction = tmp;
    }
}

pub fn snake_movement(mut snake_transform_query: Query<(&Snake, &mut Transform), With<Snake>>) {
    for (snake, mut transform) in snake_transform_query.iter_mut() {
        transform.translation += match snake.0 {
            Direction::Left => Vec3::new(-SNAKE_SPEED, 0.0, 0.0),
            Direction::Right => Vec3::new(SNAKE_SPEED, 0.0, 0.0),
            Direction::Up => Vec3::new(0.0, SNAKE_SPEED, 0.0),
            Direction::Down => Vec3::new(0.0, -SNAKE_SPEED, 0.0),
            _ => Vec3::ZERO,
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
    snake_query: Query<(&Snake, &Transform), With<Snake>>,
    bean_query: Query<(Entity, &Transform), With<Bean>>,
    resource_query: Res<SnakeResources>,
) {
    let snakes = snake_query.iter().collect::<Vec<(&Snake, &Transform)>>();
    let len = snakes.len();
    if len > 0 {
        if let Ok((bean_entity, bean_transform)) = bean_query.get_single() {
            let distance = snakes[0].1.translation.distance(bean_transform.translation);
            let snake_radius = SNAKE_SIZE / 2.0;
            // TODO: use unique size
            let bean_radius = SNAKE_SIZE / 2.0;
            if distance < snake_radius + bean_radius {
                println!("snake eat bean!");
                commands.entity(bean_entity).despawn();
                commands.spawn((
                    SpriteBundle {
                        transform: Transform::from_translation(
                            snakes[len - 1].1.translation
                                + match snakes[len - 1].0 .0 {
                                    Direction::Left => Vec3::new(SNAKE_SIZE, 0.0, 0.0),
                                    Direction::Right => Vec3::new(-SNAKE_SIZE, 0.0, 0.0),
                                    Direction::Up => Vec3::new(0.0, -SNAKE_SIZE, 0.0),
                                    Direction::Down => Vec3::new(0.0, SNAKE_SIZE, 0.0),
                                    _ => Vec3::ZERO,
                                },
                        ),
                        texture: resource_query.image.clone(),
                        ..default()
                    },
                    Snake(snakes[len - 1].0 .0),
                ));
            }
        }
    }
}
