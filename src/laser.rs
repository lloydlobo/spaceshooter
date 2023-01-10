use crate::prelude::*;

pub struct LaserDespawnEvent(pub Entity);

pub struct LaserSpawnEvent {
    /// The full position (translation + rotation) of the laser to spawn.
    pub transform: Transform,
    /// The velocity of the `Entity` emitting the laser.
    pub velocity: Velocity,
}

#[derive(Component)]
pub struct Laser {
    pub despawn_timer: Timer,
}

pub struct LaserPlugin;

impl Plugin for LaserPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LaserDespawnEvent>()
            .add_event::<LaserSpawnEvent>()
            .add_system_set(
                SystemSet::on_update(AppState::Game)
                    .with_system(laser_timeout_system)
                    .with_system(spawn_laser),
            )
            .add_system_to_stage(CoreStage::PostUpdate, laser_timeout_system);
    }
}

//----------------------------------------------------------------

fn spawn_laser(
    mut commands: Commands,
    mut laser_spawn_events: EventReader<LaserSpawnEvent>,
    handles: Res<SpriteAssets>, audios: Res<AudioAssets>,
    audio_output: Res<Audio>,
) {
    for spawn_event in laser_spawn_events.iter() {
        let transform: Transform = spawn_event.transform;

        let velocity = Velocity::linear(
            (spawn_event.velocity.linvel * Vec2::Y)
                + Vec3::truncate(transform.rotation * Vec3::Y * 500f32),
        );

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(5f32, 20f32)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(
                        transform.translation.x,
                        transform.translation.y,
                        2f32,
                    ),
                    ..default()
                },
                texture: handles.laser.clone(),
                ..default()
            },
            Laser { despawn_timer: Timer::from_seconds(2f32, TimerMode::Once) },
            ForState { states: vec![AppState::Game] },
            RigidBody::Dynamic,
            Collider::cuboid(2.5f32, 10.0f32),
            velocity,
            Sensor,
            ActiveEvents::COLLISION_EVENTS,
        ));

        audio_output.play(audios.laser_trigger.clone());
    }
}

//----------------------------------------------------------------

fn laser_timeout_system(
    mut commands: Commands, time: Res<Time>,
    gamestate: Res<State<AppGameState>>,
    mut query: Query<(Entity, &mut Laser)>,
) {
    if let &AppGameState::Game = gamestate.current() {
        query.iter_mut().for_each(|(entity, mut laser)| {
            laser.despawn_timer.tick(time.delta());
            if laser.despawn_timer.finished() {
                commands.entity(entity).despawn();
            }
        });
    }
}
