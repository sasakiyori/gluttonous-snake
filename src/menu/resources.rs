use bevy::prelude::*;

#[derive(Resource)]
pub struct MenuResources {
    pub font: Handle<Font>,
}

pub fn cache_menu_resources(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(MenuResources {
        font: asset_server.load("font/orange juice 2.0.ttf"),
    });
}
