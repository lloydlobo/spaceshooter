// [REFERENCE](https://github.com/cryscan/summer-jam/blob/master/src/utils/mod.rs)

mod collide;
mod damp;
mod interpolation;

pub use self::{
    collide::*,
    damp::*,
    interpolation::*,
};
pub use super::*;

//----------------------------------------------------------------

pub fn cleanup_system<T>(mut commands: Commands, query: Query<Entity, With<T>>)
where
    T: Component,
{
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn escape_system(mut app_state: ResMut<State<AppState>>, mut input: ResMut<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::Escape) {
        input.reset(KeyCode::Escape);
        app_state.set(AppState::StartMenu).unwrap();
    }
}

//----------------------------------------------------------------
