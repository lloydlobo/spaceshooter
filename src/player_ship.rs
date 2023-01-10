use crate::prelude::*;

/// Actions are divided in two enumerations:
/// * One for pure Player Ship actions, during effective gameplay, added on the
///   player entity itself.
/// * One for Menu actions, added as a global resource
#[derive(Actionlike, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum PlayerAction {
    Forward,
    RotateLeft,
    RotateRight,
    Fire,
}

pub struct ShipAsteroidContactEvent {
    pub ship: Entity,
    pub asteroid: Entity,
}

//-----------------------------------------------------------------------------

pub struct PlayerShipPlugin;

impl Plugin for PlayerShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<PlayerAction>::default());
        app.add_event::<ShipAsteroidContactEvent>().add_system_set(
            SystemSet::on_enter(AppState::Game).with_system(spawn_ship),
        );
    }
}

#[derive(Component)]
pub struct ExhaustEffect;

fn spawn_ship(mut commands: Commands, handles: Res<SpriteAssets>) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(30f32, 20f32)),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0f32, 0f32, 1f32),
            ..default()
        },
        texture: handles.player_ship.clone(),
        ..default()
    });
}
