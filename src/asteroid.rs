use std::{
    cmp::Ordering,
    ops::{
        Neg,
        Range,
    },
};

use bevy::utils::Duration;
use rand::rngs::ThreadRng;

use crate::prelude::*;

pub struct AsteroidSpawnEvent {
    pub size: AsteroidSize,
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub angvel: f32,
}

pub struct LaserAsteroidContactEvent {
    pub laser: Entity,
    pub asteroid: Entity,
}

//----------------------------------------------------------------

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum AsteroidSize {
    Big,
    Medium,
    Small,
}

impl AsteroidSize {
    /// Score marked when destroying an asteroid of this size.
    pub fn score(&self) -> u32 {
        match self {
            AsteroidSize::Big => 40u32,
            AsteroidSize::Medium => 30u32,
            AsteroidSize::Small => 10u32,
        }
    }

    /// Defines for each if the `Asteroid` is splitted on destruction.
    /// And the spawned sub-asteroid size and radius of spawning.
    pub fn split(&self) -> Option<(AsteroidSize, f32)> {
        match self {
            AsteroidSize::Big => Some((AsteroidSize::Medium, 10f32)),
            AsteroidSize::Medium => Some((AsteroidSize::Small, 5f32)),
            AsteroidSize::Small => None,
        }
    }
}

//----------------------------------------------------------------

#[derive(Component)]
pub struct Asteroid {
    pub size: AsteroidSize,
}

//----------------------------------------------------------------

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AsteroidSpawnEvent>()
            .add_event::<LaserAsteroidContactEvent>()
            .add_system_set(
                SystemSet::on_update(AppState::Game)
                    .with_system(arena_asteroids)
                    .with_system(spawn_asteroid_event)
                    // .with_system(asteroid_damage.after(ContactLabel)),
            );
    }
}

//----------------------------------------------------------------

fn spawn_asteroid_event(
    mut commands: Commands, mut event_reader: EventReader<AsteroidSpawnEvent>,
    handles: Res<SpriteAssets>,
) {
    for event in event_reader.iter() {
        let (sprite_handle, radius): (Handle<Image>, f32) = match event.size {
            AsteroidSize::Big => (handles.meteor_big.clone(), 101f32 / 2f32),
            AsteroidSize::Medium => (handles.meteor_med.clone(), 43f32 / 2f32),
            AsteroidSize::Small => (handles.meteor_small.clone(), 28f32 / 2f32),
        };
        commands.spawn((
            SpriteBundle {
                // No custom size, the sprite png is already at out game size.
                transform: Transform {
                    translation: Vec3::new(event.x, event.y, 1f32),
                    ..default()
                },
                texture: sprite_handle.clone(),
                ..default()
            },
            Asteroid { size: event.size },
            Damage { value: 1 },
            ForState { states: vec![AppState::Game] },
            RigidBody::Dynamic,
            Collider::ball(radius),
            ActiveEvents::COLLISION_EVENTS,
            Velocity {
                linvel: Vec2::new(event.vx, event.vy),
                angvel: event.angvel,
            },
        ));
    }
}

/// * Advance the timer by `delta` seconds. Non repeating timer will clamp at
///   duration. Repeating timer will wrap around. Will not affect paused timers.
fn arena_asteroids(
    time: Res<Time>, gamestate: Res<State<AppGameState>>,
    mut arena: ResMut<Arena>,
    mut asteroid_spawn_events: EventWriter<AsteroidSpawnEvent>,
    asteroids: Query<&Asteroid>,
) {
    let max_asteroid_count = 20;

    if gamestate.current() != &AppGameState::Game {
        return; // early exit.
    }
    arena.asteroid_spawn_timer.tick(time.delta());

    if let true = arena.asteroid_spawn_timer.finished() {
        arena.asteroid_spawn_timer.reset();

        let n_asteroid: usize = asteroids.iter().count();
        if n_asteroid.cmp(&max_asteroid_count) == Ordering::Less {
            let duration: Duration = arena.asteroid_spawn_timer.duration();
            let duration = Duration::from_secs_f32(
                (0.8f32 * duration.as_secs_f32()).max(0.1f32),
            );
            arena.asteroid_spawn_timer.set_duration(duration);

            let mut rng: ThreadRng = thread_rng();
            // 0: Top , 1: Left.
            let side: u8 = rng.gen_range(0u8..2u8);

            let (x, y): (f32, f32) = match side {
                0u8 => (
                    rng.gen_range(
                        (ARENA_WIDTH.neg() / 2f32)..(ARENA_WIDTH / 2f32),
                    ),
                    ARENA_HEIGHT / 2f32,
                ),
                _ => (
                    ARENA_WIDTH.neg() / 2f32,
                    rng.gen_range(
                        (ARENA_HEIGHT.neg() / 2f32)..{ ARENA_HEIGHT / 2f32 },
                    ),
                ),
            };
            let (rng_arena_w, rng_arena_h): (Range<f32>, Range<f32>) = (
                (ARENA_WIDTH.neg() / 4f32)..(ARENA_WIDTH / 4f32),
                (ARENA_HEIGHT.neg() / 4f32)..(ARENA_HEIGHT / 4f32),
            );
            asteroid_spawn_events.send(AsteroidSpawnEvent {
                size: AsteroidSize::Big,
                x,
                y,
                vx: rng.gen_range(rng_arena_w),
                vy: rng.gen_range(rng_arena_h),
                angvel: rng.gen_range(10f32.neg()..10f32),
            });
        }
    }
}

fn asteroid_damage() {
    todo!()
}
