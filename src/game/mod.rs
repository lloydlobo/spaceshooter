/// [Reference](https://github.com/cryscan/summer-jam/blob/master/src/game/ball.rs)
mod effects;
mod physics;
mod practice;
mod scoring;
mod tile_screen;

pub use self::{
    effects::*,
    physics::*,
    practice::*,
    scoring::*,
    tile_screen::*,
};
use crate::prelude::*;

//----------------------------------------------------------------

#[derive(Default, Clone, Copy)]
pub struct Point {
    pub position: Vec2,
    pub velocity: Vec2,
    pub time: f32,
}

//----------------------------------------------------------------

#[derive(Component)]
pub struct Trajectory {
    pub start_time: f32,
    pub points: Vec<Point>,
}

impl Default for Trajectory {
    fn default() -> Self {
        Self { start_time: 0f32, points: vec![Point::default(); PREDICT_SIZE] }
    }
}

//----------------------------------------------------------------
