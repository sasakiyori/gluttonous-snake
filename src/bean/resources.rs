use bevy::prelude::*;

#[derive(Resource)]
pub struct BeanResources {
    pub image: Handle<Image>,
}

pub fn cache_bean_resources(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(BeanResources {
        image: asset_server.load("sprite/bean.png"),
    });
}
