// https://github.com/BorisBoutillier/Kataster/blob/main/src/main.rs

// #![deny(clippy::restriction)]
#![deny(clippy::pedantic)]
#![warn(clippy::nursery)]
// #![deny(clippy::cargo)]
#![warn(dead_code)]
#![warn(unused_variables)]
#![warn(unused_must_use)]
#![deny(clippy::useless_format)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::needless_pass_by_value)]
#![allow(anonymous_parameters)]
#![allow(elided_lifetimes_in_paths)]
// Comment this out when in production.
#![allow(unused)]

mod arena;
mod assets;
mod asteroid;
mod background;
mod components;
mod config;
mod contact;
mod explosion;
mod game;
mod guardian;
mod hud;
mod laser;
mod menu;
mod particle_effects;
mod player_ship;
mod state;
mod utils;

mod prelude {
    pub use std::{
        cmp::Ordering,
        f32::consts::PI,
        ops::{
            Div,
            Neg,
            Range,
        },
        time::Duration,
    };

    pub use bevy::{
        prelude::*,
        time::FixedTimestep,
    };
    pub use bevy_rapier2d::prelude::*;
    pub use leafwing_input_manager::prelude::*;
    pub use rand::{
        rngs::ThreadRng,
        thread_rng,
        Rng,
    };

    pub use crate::{
        arena::*,
        assets::*,
        asteroid::*,
        background::*,
        components::*,
        config::*,
        contact::*,
        explosion::*,
        game::*,
        guardian::*,
        hud::*,
        laser::*,
        menu::*,
        player_ship::*,
        state::*,
        utils::*,
    };

    //----------------------------------------------------------------

    #[derive(Resource)]
    pub struct GuardianCount(pub usize);

    #[derive(Component)]
    pub struct Guardian;

    //----------------------------------------------------------------

    #[derive(Component)]
    pub struct SpriteSize(pub Vec2);

    impl From<(f32, f32)> for SpriteSize {
        fn from(value: (f32, f32)) -> Self {
            Self(Vec2::new(value.0, value.1))
        }
    }

    //----------------------------------------------------------------

    #[derive(Resource)]
    pub struct TimeScale(pub f32);
    impl Default for TimeScale {
        fn default() -> Self {
            Self(1f32)
        }
    }
    impl TimeScale {
        pub fn reset(&mut self) {
            self.0 = 1f32;
        }
    }
    //----------------------------------------------------------------
    #[derive(Component)]
    pub struct Wall;
}

use bevy::window::PresentMode;

use crate::prelude::*;

//----------------------------------------------------------------

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::rgb_u8(0, 0, 0))); // 0.01, 0.1, 0.001
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            title: "SpaceGuardian".to_string(),
            width: ARENA_WIDTH,
            height: ARENA_HEIGHT,
            present_mode: PresentMode::AutoVsync,
            ..default()
        },
        ..default()
    }));

    // Compute shaders are not supported on WASM.
    #[cfg(not(target_arch = "wasm32"))]
    {
        app.add_plugin(particle_effects::ParticleEffectsPlugin);
    }

    // Enable Rapier debug renders when compile in debug mode.
    #[cfg(debug_assertions)]
    app.add_plugin(RapierDebugRenderPlugin::default());

    app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(10f32));
    app.add_plugin(InputManagerPlugin::<MenuAction>::default());

    app.add_plugin(AssetsPlugin)
        .add_plugin(ArenaPlugin)
        .add_plugin(PlayerShipPlugin)
        .add_plugin(LaserPlugin)
        .add_plugin(GuardianPlugin)
        .add_plugin(AsteroidPlugin)
        .add_plugin(HudPlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(StatesPlugin)
        .add_plugin(ContactPlugin)
        .add_plugin(ExplosionPlugin)
        .add_plugin(BackgroundPlugin);

    app.add_state(AppState::StartMenu).add_state(AppGameState::Invalid);

    app.add_startup_system(setup_camera);
    app.add_startup_system(setup_system);

    app.run();
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn setup_system(mut commands: Commands) {
    commands.insert_resource(GuardianCount(0));

    // HACK: Temporary assignment to non existing resource.
    commands.insert_resource(WinSize { width: ARENA_WIDTH / 2f32, height: ARENA_HEIGHT / 2f32 });
}
