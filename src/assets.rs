use crate::prelude::*;

#[derive(Debug, Resource)]
pub struct SpriteAssets {
    pub player_ship: Handle<Image>,
    pub laser: Handle<Image>,
    pub meteor_big: Handle<Image>,
    pub meteor_med: Handle<Image>,
    pub meteor_small: Handle<Image>,
    pub ship_explosion: Handle<Image>,
    pub ship_contact: Handle<Image>,
    pub asteroid_explosion: Handle<Image>,
    // pub asteroid_contact: Handle<Image>,
}

#[derive(Debug, Resource)]
pub struct UiAssets {
    pub font: Handle<Font>,
    pub ship_life: UiImage,
}

#[derive(Debug, Resource)]
pub struct AudioAssets {
    pub laser_trigger: Handle<AudioSource>,
    pub ship_explosion: Handle<AudioSource>,
    pub ship_contact: Handle<AudioSource>,
    pub asteroid_explosion: Handle<AudioSource>,
    pub asteroid_contact: Handle<AudioSource>,
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
        laser: asset_server.load("laserRed07.png"),
        meteor_big: asset_server.load("meteorBrown_big1.png"),
        meteor_med: asset_server.load("meteorBrown_med1.png"),
        meteor_small: asset_server.load("meteorBrown_small1.png"),
        ship_explosion: asset_server.load("explosion01.png"),
        ship_contact: asset_server.load("explosion01.png"),
        asteroid_explosion: asset_server.load("flash00.png"),
        // asteroid_contact: asset_server.load("explosion.png"),
    });

    commands.insert_resource(AudioAssets {
        laser_trigger: asset_server.load("sfx_laser1.ogg"),
        ship_explosion: asset_server.load("Explosion_ship.ogg"),
        ship_contact: asset_server.load("Explosion.ogg"),
        asteroid_explosion: asset_server.load("Explosion.ogg"),
        asteroid_contact: asset_server.load("Explosion.ogg"),
    });

    commands.insert_resource(UiAssets {
        font: asset_server.load("kenvector_future.ttf"),
        ship_life: asset_server.load("playerLife1_red.png").into(),
    });
}
