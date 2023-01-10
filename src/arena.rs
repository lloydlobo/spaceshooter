use std::ops::Neg;

use crate::prelude::*;

pub const ARENA_WIDTH: f32 = 1280f32;
pub const ARENA_HEIGHT: f32 = 800f32;

#[derive(Debug, Resource)]
pub struct Arena {
    pub asteroid_spawn_timer: Timer,
    pub score: u32,
}

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Game).with_system(spawn_arena),
        )
        .add_system_set(
            SystemSet::on_update(AppState::Game).with_system(movement),
        );
    }
}

//----------------------------------------------------------------

/// Spawns the arena.
///
/// This function is called once when the game starts.
fn spawn_arena(
    mut commands: Commands, mut rapier_cfg: ResMut<RapierConfiguration>,
) {
    commands.insert_resource(Arena {
        asteroid_spawn_timer: Timer::from_seconds(5f32, TimerMode::Once),
        score: 0u32,
    });

    // Rapier configuration without gravity.
    rapier_cfg.gravity = Vec2::ZERO;
}

fn movement(mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in query.iter_mut() {
        let mut x: f32 = transform.translation.x;
        let mut y: f32 = transform.translation.y;
        let mut updated: bool = Updated::False.into();

        // Wrap around screen edges.
        let half_width: f32 = ARENA_WIDTH / 2f32;
        let half_height: f32 = ARENA_HEIGHT / 2f32;

        if x < half_width.neg() && velocity.linvel.x < 0f32 {
            x = half_width;
            updated = Updated::True.into();
        } else if x > half_width && velocity.linvel.x > 0f32 {
            x = half_width.neg();
            updated = Updated::True.into();
        }

        if y < half_height.neg() && velocity.linvel.y < 0f32 {
            y = half_height;
            updated = Updated::True.into();
        } else if y > half_height && velocity.linvel.y > 0f32 {
            y = half_height.neg();
            updated = Updated::True.into();
        }

        if updated {
            transform.translation.x = x;
            transform.translation.y = y;
        }
    }
}

/* /// The unary negation operator `-`.
 *
 * # Examples
 *
 * An implementation of `Neg` for `Sign`, which allows the use of `-` to
 * negate its value.
 *
 * ```
 * use std::ops::Neg;
 *
 * #[derive(Debug, PartialEq)]
 * enum Sign {
 *     Negative,
 *     Zero,
 *     Positive,
 * }
 *
 * impl Neg for Sign {
 *     type Output = Self;
 *
 *     fn neg(self) -> Self::Output {
 *         match self {
 *             Sign::Negative => Sign::Positive,
 *             Sign::Zero => Sign::Zero,
 *             Sign::Positive => Sign::Negative,
 *         }
 *     }
 * }
 *
 * // A negative positive is a negative.
 * assert_eq!(-Sign::Positive, Sign::Negative);
 * // A double negative is a positive.
 * assert_eq!(-Sign::Negative, Sign::Positive);
 * // Zero is its own negation.
 * assert_eq!(-Sign::Zero, Sign::Zero);
 * ``` */
