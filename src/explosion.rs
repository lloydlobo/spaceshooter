use crate::prelude::*;

pub enum ExplosionKind {
    ShipDead,
    ShipContact,
    LaserOnAsteroid,
    AsteroidOnGuardian,
    //GuardianOnAsteroid,
}

pub struct SpawnExplosionEvent {
    pub kind: ExplosionKind,
    pub x: f32,
    pub y: f32,
}

//----------------------------------------------------------------

pub struct ExplosionPlugin;

impl Plugin for ExplosionPlugin {
    /// Builds the explosion module.
    ///
    /// # Parameters
    ///
    /// * `app` - The application to build the module on.
    ///
    /// # Returns
    ///
    /// * `()` - Nothing.
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnExplosionEvent>()
            .add_system(animate_explosion)
            .add_system(catch_explosion_event);
    }
}

//----------------------------------------------------------------

/// Catch explosion event
///
/// # Arguments
///
/// * `commands` - Commands
/// * `event_reader` - Event reader
/// * `handles` - Sprite assets
/// * `audios` - Audio assets
/// * `audio_output` - Audio output
fn catch_explosion_event(
    mut commands: Commands, mut event_reader: EventReader<SpawnExplosionEvent>,
    handles: Res<SpriteAssets>, audios: Res<AudioAssets>, audio_output: Res<Audio>,
) {
    for event in event_reader.iter() {
        let (texture, sound, start_size, end_scale, duration) = match event.kind {
            ExplosionKind::ShipDead => (
                handles.ship_explosion.clone(),
                audios.ship_explosion.clone(),
                Vec2::new(42f32, 39f32),
                5f32,
                1.5f32,
            ),
            ExplosionKind::ShipContact => (
                handles.ship_contact.clone(),
                audios.ship_contact.clone(),
                Vec2::new(42f32, 39f32),
                2f32,
                0.5f32,
            ),
            ExplosionKind::LaserOnAsteroid => (
                handles.asteroid_explosion.clone(),
                audios.asteroid_explosion.clone(),
                Vec2::new(36f32, 32f32),
                5f32,
                1.5f32,
            ),
            // [ ]: Use assets for guardians.
            ExplosionKind::AsteroidOnGuardian => (
                handles.guardian_explosion.clone(),
                audios.guardian_explosion.clone(),
                Vec2::new(36f32, 32f32),
                5f32,
                1.5f32,
            ),
        };

        commands.spawn((
            SpriteBundle {
                sprite: Sprite { custom_size: Some(start_size), ..default() },
                transform: Transform {
                    translation: Vec3::new(event.x, event.y, 3f32),
                    ..default()
                },
                texture,
                ..default()
            },
            Explosion {
                timer: Timer::from_seconds(duration, TimerMode::Once),
                start_scale: 1f32,
                end_scale,
            },
            ForState { states: vec![AppState::Game] },
        ));

        audio_output.play(sound);
    }
}

/// Animate the explosion.
///
/// This function is called every frame.
///
/// # Arguments
///
/// * `commands` - The commands to be executed.
/// * `time` - The time.
/// * `query` - The query.
///
/// # Examples
///
/// ```ignore
/// animate_explosion(commands, time, query);
/// ```
fn animate_explosion(
    mut commands: Commands, time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut Explosion)>,
) {
    let elapsed: std::time::Duration = time.delta();

    for (entity, mut transform, mut explosion) in query.iter_mut() {
        explosion.timer.tick(elapsed);
        if explosion.timer.finished() {
            commands.entity(entity).despawn();
        } else {
            transform.scale = Vec3::splat((explosion.end_scale - (explosion).start_scale).mul_add(
                explosion.timer.elapsed_secs() / explosion.timer.duration().as_secs_f32(),
                explosion.start_scale,
            ));
        }
    }
}

// Get the new scale of the explosion.
//
// # Arguments
//
// * `e` - The explosion to get the new scale of.
//
// # Returns
//
// The new scale of the explosion.
// fn get_new_scale(e: &mut Explosion) -> f32 {
//     let diff: f32 = e.end_scale - e.start_scale;
//     let time: f32 = e.timer.elapsed_secs() /
// e.timer.duration().as_secs_f32();     e.start_scale + diff * time
// }
//
