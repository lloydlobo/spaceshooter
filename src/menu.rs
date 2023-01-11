use bevy::app::AppExit;

use crate::prelude::*;

#[derive(Component)]
pub struct DrawBlinkTimer(pub Timer);

/// List of user actions associated to menu/ui interaction.
#[derive(Actionlike, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum MenuAction {
    /// Start the game when in the start screen.
    /// Go to the start screen when in the game over screen.
    Accept,
    /// During gameplay, pause the game.
    /// Also unpause the game when in the pause screen.
    PauseUnpause,
    /// During gameplay, directly exit to the initial screen.
    ExitToMenu,
    /// During non-gameplay screens, quit the game.
    Quit,
}

//----------------------------------------------------------------

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::StartMenu).with_system(start_menu),
        )
        .add_system_set(
            SystemSet::on_enter(AppGameState::Pause).with_system(pause_menu),
        )
        .add_system_set(
            SystemSet::on_enter(AppGameState::GameOver)
                .with_system(gameover_menu),
        )
        .add_system(menu_input_system)
        .add_system(menu_blink_system)
        .add_startup_system(setup);
    }
}

pub const KEY_ACTION_PAIRS: [(KeyCode, MenuAction); 4] = [
    (KeyCode::Return, MenuAction::Accept), //GamepadButtonType::South
    (KeyCode::Escape, MenuAction::PauseUnpause), //GamepadButtonType::Start
    (KeyCode::Back, MenuAction::ExitToMenu), //GamepadButtonType::Select
    (KeyCode::Escape, MenuAction::Quit),   //GamepadButtonType::East
];

/// * Insert a mapping between `input` and `action`.
/// * Pushes a [`Command`] to the queue for inserting a [`Resource`] in the
///   [`World`] with a specific value.
fn setup(mut commands: Commands) {
    let mut input_map = InputMap::<MenuAction>::new(KEY_ACTION_PAIRS);
    input_map.insert(GamepadButtonType::Select, MenuAction::ExitToMenu);
    input_map.insert(GamepadButtonType::Start, MenuAction::PauseUnpause);
    input_map.insert(GamepadButtonType::South, MenuAction::Accept);
    input_map.insert(GamepadButtonType::East, MenuAction::Quit);

    // Insert MenuAction resources
    commands.insert_resource(input_map);
    commands.insert_resource(ActionState::<MenuAction>::default());
}

//----------------------------------------------------------------

fn start_menu(mut commands: Commands, assets: ResMut<UiAssets>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100f32), Val::Percent(100f32)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            ForState { states: vec![AppState::StartMenu] },
        ))
        .with_children(|parent| {
            parent.spawn((TextBundle {
                style: Style { ..default() },
                text: Text::from_section(
                    "SpaceShooter",
                    TextStyle {
                        font: assets.font.clone(),
                        font_size: 100f32,
                        color: Color::rgb_u8(0x00, 0xAA, 0xAA),
                    },
                ),
                ..default()
            },));

            parent.spawn((
                TextBundle {
                    style: Style { ..default() },
                    text: Text::from_section(
                        "enter",
                        TextStyle {
                            font: assets.font.clone(),
                            font_size: 50f32,
                            color: Color::rgb_u8(0x00, 0x44, 0x44),
                        },
                    ),
                    ..default()
                },
                DrawBlinkTimer(Timer::from_seconds(
                    0.5f32,
                    TimerMode::Repeating,
                )),
            ));
        });
}

fn gameover_menu(mut commands: Commands, assets: ResMut<UiAssets>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100f32), Val::Percent(100f32)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            ForState { states: vec![AppGameState::GameOver] },
        ))
        .with_children(|parent| {
            parent.spawn((TextBundle {
                style: Style { ..default() },
                text: Text::from_section(
                    "Game Over",
                    TextStyle {
                        font: assets.font.clone(),
                        font_size: 100f32,
                        color: Color::rgb_u8(0xAA, 0x22, 0x22),
                    },
                ),
                ..default()
            },));

            parent.spawn((
                TextBundle {
                    style: Style { ..default() },
                    text: Text::from_section(
                        "enter",
                        TextStyle {
                            font: assets.font.clone(),
                            font_size: 50f32,
                            color: Color::rgb_u8(0x88, 0x22, 0x22),
                        },
                    ),
                    ..default()
                },
                DrawBlinkTimer(Timer::from_seconds(
                    0.5f32,
                    TimerMode::Repeating,
                )),
            ));
        });
}

fn pause_menu(mut commands: Commands, assets: ResMut<UiAssets>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    size: Size::new(Val::Percent(100f32), Val::Percent(100f32)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            ForState { states: vec![AppGameState::Pause] },
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    style: Style { ..default() },
                    text: Text::from_section(
                        "pause",
                        TextStyle {
                            font: assets.font.clone(),
                            font_size: 100f32,
                            color: Color::rgb_u8(0xF8, 0xE4, 0x73),
                        },
                    ),
                    ..default()
                },
                DrawBlinkTimer(Timer::from_seconds(
                    0.5f32,
                    TimerMode::Repeating,
                )),
            ));
        });
}

//----------------------------------------------------------------

fn menu_blink_system(
    time: Res<Time>, mut query: Query<(&mut DrawBlinkTimer, &mut Visibility)>,
) {
    for (mut blink_timer, mut visibility) in query.iter_mut() {
        blink_timer.0.tick(time.delta());

        if blink_timer.0.finished() {
            visibility.is_visible = !visibility.is_visible;
        }
    }
}

/// * Checks if this `action` pressed since the last time /
///   [tick](ActionState::tick) was called?
// [MenuAction::Accept, MenuAction::Quit].iter().for_each(|a| {
//     match (a, menu_action_state.just_pressed(*a)) {
//         (MenuAction::Accept, true) => state.set(AppState::Game).unwrap(),
//         (MenuAction::Quit, true) => app_exit_events.send(AppExit),
//         _ => {}
//     }
// });
fn menu_input_system(
    mut state: ResMut<State<AppState>>,
    mut gamestate: ResMut<State<AppGameState>>,
    menu_action_state: Res<ActionState<MenuAction>>,
    mut rapier_cfg: ResMut<RapierConfiguration>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    let want_menu = state.current() != &AppState::StartMenu
        && menu_action_state.just_pressed(MenuAction::ExitToMenu);
    if want_menu {
        state.set(AppState::StartMenu).unwrap();
        gamestate.set(AppGameState::Invalid).unwrap();
        rapier_cfg.physics_pipeline_active = true;
    }

    match *state.current() {
        AppState::StartMenu => {
            if menu_action_state.just_pressed(MenuAction::Accept) {
                state.set(AppState::Game).unwrap();
                gamestate.set(AppGameState::Game).unwrap();
            }
            if menu_action_state.just_pressed(MenuAction::Quit) {
                app_exit_events.send(AppExit);
            }
        }
        AppState::Game => match gamestate.current() {
            AppGameState::Game => {
                if menu_action_state.just_pressed(MenuAction::PauseUnpause) {
                    gamestate.set(AppGameState::Pause).unwrap();
                    rapier_cfg.physics_pipeline_active = false;
                }
            }
            AppGameState::Pause => {
                if menu_action_state.just_pressed(MenuAction::PauseUnpause) {
                    gamestate.set(AppGameState::Game).unwrap();
                    rapier_cfg.physics_pipeline_active = true;
                }
            }
            AppGameState::GameOver => {
                if menu_action_state.just_pressed(MenuAction::Accept) {
                    state.set(AppState::StartMenu).unwrap();
                    gamestate.set(AppGameState::Invalid).unwrap();
                }
                if menu_action_state.just_pressed(MenuAction::Quit) {
                    app_exit_events.send(AppExit);
                }
            }
            AppGameState::Invalid => {}
        },
    }
}

//----------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_menu_action_pairs() {
        assert_eq!(KEY_ACTION_PAIRS.len(), 4);
    }

    #[test]
    fn test_menu_action_pairs_keycode() {
        KEY_ACTION_PAIRS.iter().enumerate().for_each(
            |(i, (keycode, action))| {
                assert_eq!(keycode, &KEY_ACTION_PAIRS[i].0);
                assert_eq!(action, &KEY_ACTION_PAIRS[i].1);
            },
        );
    }
}

//----------------------------------------------------------------
//
//----------------------------------------------------------------
//
//----------------------------------------------------------------

mod keys {
    #![allow(unused)]

    use std::{
        iter::Enumerate,
        slice::Iter,
    };

    use crate::prelude::*;

    pub const ACTIONS: [MenuAction; 4] = [
        MenuAction::Accept,
        MenuAction::PauseUnpause,
        MenuAction::ExitToMenu,
        MenuAction::Quit,
    ];

    enum ActionMode {
        KeyCode,
        GamepadButton,
    }

    struct KeyPad {
        key_code: Option<Vec<(KeyCode, MenuAction)>>,
        gamepad_button: Option<Vec<(GamepadButtonType, MenuAction)>>,
    }

    fn get_keypad(actions: [MenuAction; 4], action_type: ActionMode) -> KeyPad {
        let actions: Enumerate<Iter<MenuAction>> = actions.iter().enumerate();
        match action_type {
            ActionMode::KeyCode => KeyPad {
                key_code: Some(
                    actions
                        .map(|(i, a)| (get_scheme(a).0, ACTIONS[i]))
                        .collect(),
                ),
                gamepad_button: None,
            },
            ActionMode::GamepadButton => KeyPad {
                key_code: None,
                gamepad_button: Some(
                    actions
                        .map(|(i, a)| (get_scheme(a).1, ACTIONS[i]))
                        .collect(),
                ),
            },
        }
    }

    fn get_scheme(menu_action: &MenuAction) -> (KeyCode, GamepadButtonType) {
        match menu_action {
            MenuAction::Accept => (KeyCode::Return, GamepadButtonType::South),
            MenuAction::PauseUnpause => {
                (KeyCode::Escape, GamepadButtonType::Start)
            }
            MenuAction::ExitToMenu => {
                (KeyCode::Back, GamepadButtonType::Select)
            }
            MenuAction::Quit => (KeyCode::Escape, GamepadButtonType::East),
        }
    }
}

//----------------------------------------------------------------
//
//----------------------------------------------------------------
//
//----------------------------------------------------------------
