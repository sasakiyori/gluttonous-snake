use bevy::prelude::*;

#[derive(Resource)]
pub struct SnakeResources {
    pub image: Handle<Image>,
}

pub fn cache_snake_resources(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(SnakeResources {
        image: asset_server.load("snake.png"),
    });
}
