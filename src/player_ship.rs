use std::{
    ops::Neg,
    time::Duration,
};

use crate::prelude::*;

pub const START_LIFE: u32 = 3u32;
const INVINCIBLE_TIME: f32 = 2f32;
const MAX_INVINCIBLE_TIME: f32 = 5f32;

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

/// The ship used by a `Player`.
#[derive(Component)]
pub struct Ship {
    /// Ship rotation speed in `rad/s`.
    pub rotation_speed: f32,
    /// Ship thrust N (Newton).
    pub thrust: f32,
    /// Ship life health points.
    pub life: u32,
    /// Cannon auto-fire timer.
    pub cannon_timer: Timer,
    /// Id of the controlling player. `Player` 1 or `Player` 2.
    pub player_id: u32,
    /// Timer triggered after being hit that provides short-term invincibility.
    pub invincible_timer: Timer,
    /// Total duration of invincibility, accumulating when renewed.
    pub invincible_time_secs: f32,
}

#[derive(Component, Clone, Copy)]
pub struct Damage {
    pub value: u32,
}

pub struct ShipAsteroidContactEvent {
    pub ship: Entity,
    pub asteroid: Entity,
}

//----------------------------------------------------------------

pub struct PlayerShipPlugin;

impl Plugin for PlayerShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<PlayerAction>::default());
        app.add_event::<ShipAsteroidContactEvent>()
            .add_system_set(
                SystemSet::on_enter(AppState::Game).with_system(spawn_ship),
            )
            .add_system_set(
                SystemSet::on_update(AppState::Game)
                    .with_system(ship_input_system)
                    .with_system(ship_dampening_system),
            );
    }
}

//----------------------------------------------------------------

/// Tag component to update the exhaust particle effect with speed.
#[derive(Component)]
pub struct ExhaustEffect;

fn spawn_ship(mut commands: Commands, handles: Res<SpriteAssets>) {
    let mut input_map = InputMap::new([
        (KeyCode::W, PlayerAction::Forward),
        (KeyCode::Up, PlayerAction::Forward),
        (KeyCode::A, PlayerAction::RotateLeft),
        (KeyCode::Left, PlayerAction::RotateLeft),
        (KeyCode::D, PlayerAction::RotateRight),
        (KeyCode::Right, PlayerAction::RotateRight),
        (KeyCode::Space, PlayerAction::Fire),
    ]);

    input_map.insert(GamepadButtonType::South, PlayerAction::Fire);
    input_map.insert(
        SingleAxis::positive_only(GamepadAxisType::LeftStickY, 0.4f32),
        PlayerAction::Forward,
    );
    input_map.insert(
        SingleAxis::negative_only(GamepadAxisType::LeftStickY, 0.4f32.neg()),
        PlayerAction::Forward,
    );
    input_map.insert(
        SingleAxis::positive_only(GamepadAxisType::LeftStickX, 0.4f32),
        PlayerAction::RotateRight,
    );
    input_map.insert(
        SingleAxis::negative_only(GamepadAxisType::LeftStickX, 0.4f32.neg()),
        PlayerAction::RotateLeft,
    );

    let mut invincible_timer =
        Timer::from_seconds(INVINCIBLE_TIME, TimerMode::Once);
    // Immediately consume the timer, we don't want invincibility at creation.
    invincible_timer.tick(Duration::from_secs_f32(INVINCIBLE_TIME));

    commands.spawn((
        SpriteBundle {
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
        },
        Ship {
            rotation_speed: 3f32,
            thrust: 60f32,
            life: START_LIFE,
            cannon_timer: Timer::from_seconds(0.2f32, TimerMode::Once),
            player_id: 1,
            invincible_timer,
            invincible_time_secs: 0f32,
        },
        ForState { states: vec![AppState::Game] },
        RigidBody::Dynamic,
        Collider::ball(13.5f32),
        ExternalImpulse::default(),
        Velocity::linear(Vec2::ZERO),
        ActiveEvents::COLLISION_EVENTS,
        InputManagerBundle::<PlayerAction> {
            action_state: ActionState::default(),
            input_map,
        },
    ));
}

//----------------------------------------------------------------

fn ship_dampening_system(
    time: Res<Time>, mut query: Query<&mut Velocity, With<Ship>>,
) {
    for mut velocity in query.iter_mut() {
        let elapsed: f32 = time.delta_seconds();
        velocity.angvel *= 0.1f32.powf(elapsed);
        velocity.linvel *= 0.4f32.powf(elapsed);
    }
}

fn ship_input_system(
    gamestate: Res<State<AppGameState>>,
    mut laser_spawn_events: EventWriter<LaserSpawnEvent>,
    mut query: Query<(
        &ActionState<PlayerAction>,
        &mut ExternalImpulse,
        &mut Velocity,
        &Transform,
        &mut Ship,
    )>,
) {
    if gamestate.current() == &AppGameState::Game {
        for (action_state, mut impulse, mut velocity, transform, mut ship) in
            query.iter_mut()
        {
            let thrust: f32 = if action_state.pressed(PlayerAction::Forward) {
                1f32
            } else {
                0f32
            };
            let rotation = if action_state.pressed(PlayerAction::RotateLeft) {
                1
            } else if action_state.pressed(PlayerAction::RotateRight) {
                1.neg()
            } else {
                0
            };

            let fire: bool = action_state.pressed(PlayerAction::Fire);

            if rotation != 0 {
                velocity.angvel = rotation as f32 * ship.rotation_speed;
            }

            impulse.impulse = (transform.rotation
                * (Vec3::Y * thrust * ship.thrust))
                .truncate();

            if fire && ship.cannon_timer.finished() {
                laser_spawn_events.send(LaserSpawnEvent {
                    transform: *transform,
                    velocity: *velocity,
                });

                ship.cannon_timer.reset();
            }
        }
    }
}

//----------------------------------------------------------------
