// [REFERENCE](https://github.com/cryscan/summer-jam/blob/master/src/config.rs)

use crate::prelude::*;

//----------------------------------------------------------------
// player_ship

pub const START_LIFE: u32 = 3u32;
pub const INVINCIBLE_TIME: f32 = 2f32;
pub const MAX_INVINCIBLE_TIME: f32 = 5f32;

//----------------------------------------------------------------

pub const ARENA_WIDTH: f32 = 1280f32;
pub const ARENA_HEIGHT: f32 = 800f32;
pub const ARENA_PADDING: f32 = 20f32;

//----------------------------------------------------------------
// fire_guardian

pub const MAX_GUARDIAN_COUNT: usize = 4usize;
pub const GUARDIAN_SIZE: (f32, f32) = (97f32, 97f32);
pub const SPRITE_SCALE: f32 = 1f32;

//----------------------------------------------------------------
//guardian

pub const GUARDIAN_RADIUS: GuardianRadius =
    GuardianRadius { big: 256f32 / 2f32, med: 158f32 / 2f32, small: 97f32 / 2f32 };
//----------------------------------------------------------------

pub const GUARDIAN_FIRE_MIN_SPEED: f32 = 1f32;
pub const GUARDIAN_FIRE_MAX_SPEED: f32 = 1f32;
pub const GUARDIAN_FIRE_NORMAL_SPEED: f32 = 1f32;
pub const GUARDIAN_FIRE_DAMP: f32 = 1f32;
pub const GUARDIAN_FIRE_HIT_RANGE_VERTICAL: f32 = 144f32;
pub const GUARDIAN_FIRE_HIT_RANGE_HORIZONTAL: f32 = 144f32;
pub const GUARDIAN_FIRE_HIT_SPEED_THRESHOLD: f32 = -0f32;

//----------------------------------------------------------------

pub const GUARDIAN_FORMATION_MEMBERS_MAX: u32 = 4u32;

//----------------------------------------------------------------

pub const TIME_STEP: f32 = 1f32 / 60f32;
pub const BASE_SPEED: f32 = 500f32;

//----------------------------------------------------------------

pub const PREDICT_SIZE: usize = 100usize;
pub const PREDICT_TIME_STEP: f32 = 0.01f32;
pub const AI_TIME_STEP: f32 = 0.1f32;

//----------------------------------------------------------------

pub const UI_WIDTH: f32 = 120f32;
pub const WALL_SPACING: f32 = 5f32;
pub const GUARDIAN_SPACING: f32 = 9f32;
pub const TILE_SIZE: f32 = 20f32;
pub const HALF_TILE: f32 = TILE_SIZE / 2f32;
pub const COLUMN_SIZE: f32 = TILE_SIZE * 3f32;

//----------------------------------------------------------------

pub const PADDING: usize = 2usize;

//----------------------------------------------------------------

pub const SCREEN_X: f32 = ARENA_WIDTH / -2f32 + ARENA_PADDING;
pub const SCREEN_Y: f32 = ARENA_HEIGHT / -2f32 + ARENA_PADDING;

//----------------------------------------------------------------

//----------------------------------------------------------------

//----------------------------------------------------------------
// game/scoring

pub const ASTEROID_SCORE_HIT_POINTS_SMALL: u32 = 40u32;
pub const ASTEROID_SCORE_HIT_POINTS_MEDIUM: u32 = 20u32;
pub const ASTEROID_SCORE_HIT_POINTS_BIG: u32 = 10u32;
pub const GUARDIAN_SCORE_HIT_POINTS_SMALL: u32 = 40u32;
pub const GUARDIAN_SCORE_HIT_POINTS_MEDIUM: u32 = 20u32;
pub const GUARDIAN_SCORE_HIT_POINTS_BIG: u32 = 10u32;

//----------------------------------------------------------------
