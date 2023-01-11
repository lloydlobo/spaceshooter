use crate::prelude::*;

pub enum ExplosionKind {
    ShipDead,
    ShipContact,
    LaserOnAsteroid,
}

pub struct SpawnExplosionEvent {
    pub kind: ExplosionKind,
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Explosion {
    timer: Timer,
    start_scale: f32,
    end_scale: f32,
}

//----------------------------------------------------------------

pub struct ExplosionPlugin;

impl Plugin for ExplosionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnExplosionEvent>()
            .add_system(animate_explosion)
            .add_system(catch_explosion_event);
    }
}

//----------------------------------------------------------------

fn catch_explosion_event(
    mut commands: Commands, mut event_reader: EventReader<SpawnExplosionEvent>,
    handles: Res<SpriteAssets>, audios: Res<AudioAssets>,
    audio_output: Res<Audio>,
) {
    for event in event_reader.iter() {
        let (texture, sound, start_size, end_scale, duration) = match event.kind
        {
            ExplosionKind::ShipDead => (
                handles.ship_explosion.clone(),
                audios.ship_explosion.clone(),
                Vec2::new(42f32, 39f32),
                5.0f32,
                1.5f32,
            ),
            ExplosionKind::ShipContact => (
                handles.ship_contact.clone(),
                audios.ship_contact.clone(),
                Vec2::new(42f32, 39f32),
                2.0f32,
                0.5f32,
            ),
            ExplosionKind::LaserOnAsteroid => (
                handles.asteroid_explosion.clone(),
                audios.asteroid_explosion.clone(),
                Vec2::new(36f32, 32f32),
                5.0f32,
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

fn animate_explosion(
    mut commands: Commands, time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut Explosion)>,
) {
    let elapsed: std::time::Duration = time.delta();
    for (entity, mut transform, mut explosion) in query.iter_mut() {
        explosion.timer.tick(elapsed);
    }
}
