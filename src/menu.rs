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

const KEY_ACTION_PAIRS: [(KeyCode, MenuAction); 4] = [
    (KeyCode::Return, MenuAction::Accept), //GamepadButtonType::South
    (KeyCode::Escape, MenuAction::PauseUnpause), //GamepadButtonType::Start
    (KeyCode::Back, MenuAction::ExitToMenu), //GamepadButtonType::Select
    (KeyCode::Escape, MenuAction::Quit),   //GamepadButtonType::East
];

//-----------------------------------------------------------------------------

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::StartMenu).with_system(start_menu),
        );
    }
}

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

mod tests {
    use super::*;

    #[test]
    fn test_menu_action_pairs() {
        assert_eq!(KEY_ACTION_PAIRS.len(), 4);
    }

    // [ ]: Implement keycode with action.
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

//-----------------------------------------------------------------------------

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
