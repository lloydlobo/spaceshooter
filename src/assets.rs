use crate::prelude::*;

#[derive(Debug, Resource)]
pub struct SpriteAssets {
    pub player_ship: Handle<Image>,
}

#[derive(Debug, Resource)]
pub struct UiAssets {
    pub font: Handle<Font>,
    // pub ship_life:UiImage,
}

//----------------------------------------------------------------

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(SpriteAssets {
        player_ship: asset_server.load("playerShip2_red.png"),
    });
    commands.insert_resource(UiAssets {
        font: asset_server.load("kenvector_future.ttf"),
    });
}
