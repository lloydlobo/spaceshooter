use crate::prelude::*;

//----------------------------------------------------------------

pub struct GuardianRadius {
    pub big: f32,
    pub med: f32,
    pub small: f32,
}

pub struct GuardianSpawnEvent {
    pub size: GuardianSize,
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub angvel: f32,
}

// Asteroid hits guardian. Guardian gets damaged.
pub struct AsteroidGuardianContactEvent {
    pub asteroid: Entity,
    pub guardian: Entity,
}

//----------------------------------------------------------------

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum GuardianSize {
    Big,
    Medium,
    Small,
}

impl GuardianSize {
    /// Score marked & deduced when a guardian of this size gets destroyed.
    pub const fn score(self) -> u32 {
        match self {
            Self::Big => Score::Guardian(Self::Big).score(),
            Self::Medium => Score::Guardian(Self::Medium).score(),
            Self::Small => Score::Guardian(Self::Small).score(),
        }
    }

    /// Defines for each if the `Guardian` is splitted on destruction.
    /// And spawned sub-guardian size (`Self`) and radius (`f32`) of spawning.
    pub const fn split(self) -> Option<(Self, f32)> {
        match self {
            Self::Big => Some((Self::Medium, 10f32)),
            Self::Medium => Some((Self::Small, 5f32)),
            Self::Small => None,
        }
    }
}

//----------------------------------------------------------------

#[derive(Component)]
pub struct Guardian {
    pub size: GuardianSize,
}

//----------------------------------------------------------------

pub struct GuardianPlugin;

impl Plugin for GuardianPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GuardianSpawnEvent>()
            .add_event::<AsteroidGuardianContactEvent>()
            // .add_event::<GuardianAsteroidContactEvent>()
            .add_system_set(
                SystemSet::on_update(AppState::Game)
                    .with_system(arena_guardians)
                    // .with_system(spawn_prime)
                    .with_system(spawn_guardian_event)
                    .with_system(guardian_dampening_system)
                    .with_system(guardian_damage.after(ContactLabel)),
            );
    }
}

fn spawn_prime(
    mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Rectangle
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25f32, 0.25f32, 0.25f32),
            custom_size: Some(Vec2::new(50f32, 100f32)),
            ..default()
        },
        ..default()
    });
}

//----------------------------------------------------------------

fn arena_guardians(
    time: Res<Time>, gamestate: Res<State<AppGameState>>, mut arena: ResMut<Arena>,
    mut guardian_spawn_events: EventWriter<GuardianSpawnEvent>, guardians: Query<&Guardian>,
) {
    if gamestate.current() != &AppGameState::Game {
        return;
    }
    arena.guardian_spawn_timer.tick(time.delta());
    if !arena.guardian_spawn_timer.finished() {
        return;
    }
    arena.guardian_spawn_timer.reset();
    let n_guardian: usize = guardians.iter().count();
    if matches!(n_guardian.cmp(&MAX_GUARDIAN_COUNT), Ordering::Greater) {
        return;
    }

    let duration: f32 = arena.guardian_spawn_timer.duration().as_secs_f32();
    let duration: f32 = (0.8f32 * duration).max(0.1f32);
    arena.guardian_spawn_timer.set_duration(Duration::from_secs_f32(duration));

    let mut rng: ThreadRng = thread_rng();
    // 0:Top, 1:Left.
    let side: u8 = rng.gen_range(0u8..2u8);
    let (x, y): (f32, f32) = match side {
        0u8 => (
            rng.gen_range((ARENA_WIDTH.neg().div(2f32))..(ARENA_WIDTH.div(2f32))),
            ARENA_HEIGHT.div(2f32),
        ),
        _ => (
            ARENA_WIDTH.neg().div(2f32),
            rng.gen_range((ARENA_HEIGHT.neg().div(2f32))..ARENA_HEIGHT.div(2f32)),
        ),
    };
    let (rng_arena_w, rng_arena_h): (Range<f32>, Range<f32>) = (
        (ARENA_WIDTH.neg().div(4f32))..(ARENA_WIDTH.div(4f32)),
        (ARENA_HEIGHT.neg().div(4f32))..(ARENA_HEIGHT.div(4f32)),
    );

    guardian_spawn_events.send(GuardianSpawnEvent {
        size: GuardianSize::Medium,
        x,
        y,
        vx: rng.gen_range(rng_arena_w), // Drastically reduce velocity
        vy: rng.gen_range(rng_arena_h), // Drastically reduce velocity
        angvel: rng.gen_range(8f32.neg()..8f32),
    });
}

//----------------------------------------------------------------

fn guardian_dampening_system(time: Res<Time>, mut query: Query<&mut Velocity, With<Guardian>>) {
    for mut velocity in query.iter_mut() {
        let elapsed: f32 = time.delta_seconds();
        velocity.angvel *= 0.2f32.powf(elapsed); //0.1f32...
        velocity.linvel *= 0.4f32.powf(elapsed); //0.4f32...
    }
}

/// Match and associate sprite texture with each guardian size.
fn spawn_guardian_event(
    mut commands: Commands, mut event_reader: EventReader<GuardianSpawnEvent>,
    handles: Res<SpriteAssets>,
) {
    for event in event_reader.iter() {
        let (sprite_handle, radius) = match event.size {
            GuardianSize::Big => (handles.guardian_big.clone(), GUARDIAN_RADIUS.big),
            GuardianSize::Medium => (handles.guardian_med.clone(), GUARDIAN_RADIUS.med),
            GuardianSize::Small => (handles.guardian_small.clone(), GUARDIAN_RADIUS.small),
        };
        commands.spawn((
            SpriteBundle {
                // No custom size, the sprite png is already at out game size.
                transform: Transform {
                    translation: Vec3::new(event.x.div(4f32), event.y.div(4f32), 1f32),
                    ..default()
                },
                texture: sprite_handle.clone(),
                ..default()
            },
            Guardian { size: event.size },
            Damage { value: 0u32 }, // Damage to player_ship.
            ForState { states: vec![AppState::Game] },
            RigidBody::Dynamic,
            // RigidBody::Fixed,
            Collider::ball(radius),
            ActiveEvents::COLLISION_EVENTS, // CONTACT_FORCE_EVENTS
            // Velocity { linvel: Vec2::new(event.vx, event.vy), angvel: event.angvel },
            Velocity { linvel: Vec2::ZERO, angvel: 0f32 },
        ));
    }
}

fn guardian_damage(
    mut commands: Commands, mut arena: ResMut<Arena>,
    mut asteroid_guardian_contact_event: EventReader<AsteroidGuardianContactEvent>,
    mut explosion_spawn_events: EventWriter<SpawnExplosionEvent>,
    mut guardian_spawn_events: EventWriter<GuardianSpawnEvent>, transforms: Query<&Transform>,
    guardians: Query<(&Guardian, &Transform, &Velocity)>,
) {
    for event in asteroid_guardian_contact_event.iter() {
        let asteroid_transform: &Transform =
            transforms.get(event.asteroid).expect("should get transform events of asteroid");
        let (guardian, guardian_transform, guardian_velocity) = guardians
            .get(event.guardian)
            .expect("should get guardian: Self, transform, velocity from Query");

        {
            explosion_spawn_events.send(SpawnExplosionEvent {
                kind: ExplosionKind::AsteroidOnGuardian,
                x: asteroid_transform.translation.x,
                y: asteroid_transform.translation.y,
            });

            if let Some((size, radius)) = guardian.size.split() {
                let mut rng: ThreadRng = thread_rng();
                for _ in 0..rng.gen_range(1u8..4u8) {
                    guardian_spawn_events.send(GuardianSpawnEvent {
                        size,
                        x: guardian_transform.translation.x + rng.gen_range(radius.neg()..radius),
                        y: guardian_transform.translation.y + rng.gen_range(radius.neg()..radius),
                        vx: rng.gen_range((ARENA_WIDTH.neg() / radius)..(ARENA_WIDTH / radius)),
                        vy: rng.gen_range((ARENA_HEIGHT.neg() / radius)..(ARENA_HEIGHT / radius)),
                        angvel: guardian_velocity.angvel,
                    });
                }
            }
        }

        commands.entity(event.asteroid).despawn();
        commands.entity(event.guardian).despawn();
    }
}
