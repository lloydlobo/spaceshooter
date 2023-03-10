use bevy::ecs::schedule::StateData;

use crate::prelude::*;

/// Main state enumerated differentiating `Menu` from `Game` 'scenes'.
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum AppState {
    StartMenu,
    Game,
}

/// Game state enum, differentiating several phase of the game
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum AppGameState {
    /// Invalid used when AppState is NOT Game
    Invalid,
    Game,
    Pause,
    GameOver,
}

//----------------------------------------------------------------

pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        for state in [AppState::StartMenu, AppState::Game] {
            app.add_system_set(
                SystemSet::on_enter(state).with_system(state_enter_despawn::<AppState>),
            );
        }

        for state in
            [AppGameState::Invalid, AppGameState::Game, AppGameState::Pause, AppGameState::GameOver]
        {
            app.add_system_set(
                SystemSet::on_enter(state).with_system(state_enter_despawn::<AppGameState>),
            );
        }
    }
}

fn state_enter_despawn<T>(
    mut commands: Commands, state: ResMut<State<T>>, query: Query<(Entity, &ForState<T>)>,
) where
    T: StateData,
{
    for (entity, for_state) in &mut query.iter() {
        if !for_state.states.contains(state.current()) {
            commands.entity(entity).despawn_recursive();
        }
    }
}
