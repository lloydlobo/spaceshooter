use bevy::ecs::schedule::StateData;

use crate::prelude::*;

/// Main state enumerated differentiating `Menu` from `Game` 'scenes'.
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum AppState {
    StartMenu,
    Game,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub(crate) enum AppGameState {
    /// Invalid used when AppState is NOT Game
    Invalid,
    Game,
    Pause,
    GameOver,
}
