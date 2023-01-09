#![allow(clippy::too_many_arguments)]

mod arena;
mod assets;
mod background;
mod enums;
mod state;

// https://github.com/BorisBoutillier/Kataster/blob/main/src/main.rs

mod prelude {
    pub use bevy::prelude::*;
    pub use bevy_rapier2d::prelude::*;
    pub use leafwing_input_manager::prelude::*;
    pub use rand::{
        thread_rng,
        Rng,
    };

    pub use crate::{
        arena::*,
        assets::*,
        background::*,
        enums::*,
        state::*,
    };
}

use bevy::window::PresentMode;

use crate::prelude::*;

fn main() {
    println!("Hello, world!");
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::rgb_u8(0, 0, 0))); // 0.01, 0.1, 0.001
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            width: ARENA_WIDTH,
            height: ARENA_HEIGHT,
            title: "SpaceShooter".to_string(),
            present_mode: PresentMode::AutoVsync,
            ..default()
        },
        ..default()
    }));

    // Compute shaders are not supported on WASM.
    #[cfg(not(target_arch = "wasm32"))]
    // {
    // app.add_plugin(particle_effects::ParticleEffectsPlugin);
    // }

    // Enable Rapier debug renders when compile in debug mode.
    #[cfg(debug_assertions)]
    app.add_plugin(RapierDebugRenderPlugin::default());

    app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(10f32));
    // app.add_plugin(InputManagerPlugin::<MenuAction>::default());

    app.add_plugin(AssetsPlugin)
        .add_plugin(ArenaPlugin)
        // .add_plugin(plugin)
        // .add_plugin(plugin)
        // .add_plugin(plugin)
        // .add_plugin(plugin)
        // .add_plugin(plugin)
        // .add_plugin(plugin)
        // .add_plugin(plugin)
        // .add_plugin(plugin)
        .add_plugin(BackgroundPlugin);

    app.add_state(AppState::StartMenu).add_state(AppGameState::Invalid);

    app.add_startup_system(setup_camera);
    app.run();
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}