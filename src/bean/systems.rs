use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::random;

use super::components::Bean;
use super::cst::BEAN_RADIUS;
use super::resources::BeanResources;

use crate::snake::components::Snake;
use crate::snake::cst::{SNAKE_RADIUS, SNAKE_SPEED};

pub fn spawn_bean(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    snake_query: Query<&Transform, With<Snake>>,
    bean_query: Query<&Bean>,
    resource_query: Res<BeanResources>,
) {
    // no need to spawn if bean still exists
    if !bean_query.is_empty() {
        return;
    }

    let window = window_query.get_single().unwrap();
    let mut x: f32 = 0.0;
    let mut y: f32 = 0.0;
    let mut collision: bool;

    // only try 10 times :)
    for _ in 0..10 {
        collision = false;
        // let bean and snake be in the same grid
        x = (random::<f32>() * (window.width() - BEAN_RADIUS) / SNAKE_SPEED).floor() * SNAKE_SPEED;
        y = (random::<f32>() * (window.height() - BEAN_RADIUS) / SNAKE_SPEED).floor() * SNAKE_SPEED;

        // check collision
        for snake_transform in snake_query.iter() {
            if snake_transform.translation.distance(Vec3 { x, y, z: 0.0 })
                < SNAKE_RADIUS + BEAN_RADIUS
            {
                collision = true;
                break;
            }
        }
        if !collision {
            break;
        }
    }

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(x, y, 0.0),
            // why use the clone of the resource but not directly load the asset directly?
            // same issue as: https://github.com/bevyengine/bevy/discussions/8288
            texture: resource_query.image.clone(),
            ..default()
        },
        Bean {},
    ));
}

pub fn despawn_bean(mut commands: Commands, bean_query: Query<Entity, With<Bean>>) {
    for entity in bean_query.iter() {
        commands.entity(entity).despawn();
    }
}
