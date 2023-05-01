use bevy::{prelude::*, window::PrimaryWindow};

const SNAKE_SPEED: f32 = 500.0;
const SNAKE_SIZE: f32 = 18.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_snake)
        .add_system(snake_movement)
        .add_system(snake_dead_check)
        .run();
}

#[derive(Component)]
struct Snake;

fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

fn spawn_snake(
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
        Snake {},
    ));
}

fn snake_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut snake_query: Query<&mut Transform, With<Snake>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = snake_query.get_single_mut() {
        let mut direction = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * SNAKE_SPEED * time.delta_seconds();
    }
}

fn snake_dead_check(
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
