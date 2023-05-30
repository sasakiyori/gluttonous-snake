use bevy::prelude::*;

pub const BEAN_SIZE: f32 = 18.0;
pub const BEAN_RADIUS: f32 = BEAN_SIZE / 2.0;

#[derive(Resource)]
pub struct BeanResources {
    pub image: Handle<Image>,
}

pub fn cache_bean_resources(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(BeanResources {
        image: asset_server.load("bean.png"),
    });
}
