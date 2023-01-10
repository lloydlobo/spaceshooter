use bevy::ecs::schedule::StateData;

use crate::prelude::*;

/// `Component` to tag an entity as only needed in some of the states
#[derive(Debug, Component)]
pub struct ForState<T> {
    pub states: Vec<T>,
}

/// Main state enumerated differentiating `Menu` from `Game` 'scenes'.
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum AppState {
    StartMenu,
    Game,
}

/// Game state enum, differentiating several phase of the game
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub(crate) enum AppGameState {
    /// Invalid used when AppState is NOT Game
    Invalid,
    Game,
    Pause,
    GameOver,
}

//-----------------------------------------------------------------------------

pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        const APP_STATES: [AppState; 2] = [AppState::StartMenu, AppState::Game];
        const APP_GAME_STATES: [AppGameState; 4] = [
            AppGameState::Invalid,
            AppGameState::Game,
            AppGameState::Pause,
            AppGameState::GameOver,
        ];

        for state in APP_STATES.into_iter() {
            app.add_system_set(
                SystemSet::on_enter(state)
                    .with_system(state_enter_despawn::<AppState>),
            );
        }
        for state in APP_GAME_STATES.into_iter() {
            app.add_system_set(
                SystemSet::on_enter(state)
                    .with_system(state_enter_despawn::<AppGameState>),
            );
        }
    }
}

fn state_enter_despawn<T>(
    mut commands: Commands, state: ResMut<State<T>>,
    query: Query<(Entity, &ForState<T>)>,
) where
    T: StateData,
{
    for (entity, for_state) in &mut query.iter() {
        if !for_state.states.contains(state.current()) {
            commands.entity(entity).despawn_recursive();
        }
    }
}
