use std::ops::Neg;

use bevy_hanabi::prelude::*;

use crate::prelude::*;

/// `Plugin` that adds particle effects at different point in the game.
/// All particle effects are handled in a separate plugin to be easily disabled
/// when targeting WASM.
pub struct ParticleEffectsPlugin;

impl Plugin for ParticleEffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(HanabiPlugin)
            .add_system(add_thrust_particles_to_ship)
            .add_system(update_thrust_particles);
    }
}

//----------------------------------------------------------------

/// Add a Particle Effect to every new created `Ship` `Entity`.
fn add_thrust_particles_to_ship(
    mut commands: Commands, mut effects: ResMut<Assets<EffectAsset>>,
    added_ships: Query<Entity, Added<Ship>>,
) {
    for ship_entity in added_ships.iter() {
        // For `Ship` exhaust, we store a particle effect on the player

        // Gradient for particle color evolution
        let mut gradient = Gradient::<Vec4>::new();
        gradient.add_key(0.0f32, Vec4::new(0.5f32, 0.4f32, 0.7f32, 0.8f32));
        gradient.add_key(0.5f32, Vec4::new(1.0f32, 0.8f32, 0.0f32, 0.8f32));
        gradient.add_key(1.0f32, Vec4::ZERO);

        let effect = effects.add(
            EffectAsset {
                name: "Exhaust".to_string(),
                capacity: 16_024,
                spawner: Spawner::once(10f32.into(), false),
                //spawner: Spawner::rate(500f32.into()),
                z_layer_2d: 10f32, // Z coordinate used as the sort key
                ..default()
            } // A modifier to set the lifetime of all particles
            .init(ParticleLifetimeModifier { lifetime: 0.1f32 })
            // Modifier spawns particles inside a truncated 3D cone.
            .init(PositionCone3dModifier {
                height: 5f32.neg(),
                base_radius: 2f32,
                top_radius: 1f32,
                speed: Value::Uniform((100f32, 400f32)),
                dimension: ShapeDimension::Volume,
            })
            .render(ColorOverLifetimeModifier { gradient })
            .render(SizeOverLifetimeModifier {
                gradient: Gradient::constant(Vec2::splat(2f32)),
            }),
        );

        commands.entity(ship_entity).add_children(|parent| {
            parent.spawn((
                ParticleEffectBundle {
                    effect: ParticleEffect::new(effect),
                    transform: Transform::from_translation(Vec3::new(
                        0f32,
                        3f32.neg(),
                        0f32,
                    )),
                    ..default()
                },
                ExhaustEffect,
            ));
        });
    }
}

/// Trigger a new particle spawning whenever the `Ship` `Impulse` is non-0.
fn update_thrust_particles(
    impulse: Query<(&ExternalImpulse, &Children), Changed<ExternalImpulse>>,
    mut exhaust_effect: Query<&mut ParticleEffect, With<ExhaustEffect>>,
) {
    for (impulse, children) in impulse.iter() {
        if impulse.impulse.length() != 0f32 {
            for &child in children.iter() {
                if let Ok(mut effect) = exhaust_effect.get_mut(child) {
                    if let Some(spawner) = effect.maybe_spawner() {
                        spawner.reset();
                    }
                }
            }
        }
    }
}
