// [REFERENCE](https://github.com/wichops/bevy_retro_racing/blob/main/src/tile_screen.rs)

use crate::prelude::*;

//----------------------------------------------------------------

#[derive(Default)]
pub struct TileScreen {}

impl TileScreen {
    #[allow(clippy::cast_precision_loss)]
    pub fn column_to_coord(column: usize) -> f32 {
        let padding = PADDING as f32;
        let column = column as f32;

        // `SCREEN_X+(column*COLUMN_SIZE)+(HALF_TILE*3f32)+TILE_SIZE*padding`
        TILE_SIZE.mul_add(padding, HALF_TILE.mul_add(3f32, column.mul_add(COLUMN_SIZE, SCREEN_X)))
    }

    pub const fn tile_scale() -> Vec3 {
        Vec3::new(0.85f32, 0.85f32, 0f32)
    }
}

//----------------------------------------------------------------
