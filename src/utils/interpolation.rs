// [REFERENCE](https://github.com/cryscan/summer-jam/blob/master/src/utils/interpolation.rs)

use crate::prelude::*;

//----------------------------------------------------------------

pub trait Interpolation {
    fn lerp(self, end: Self, factor: f32) -> Self;
}

macro_rules! impl_interpolation {
    ($type:ty) => {
        impl Interpolation for $type {
            #[allow(clippy::suboptimal_flops)]
            fn lerp(self, end: Self, factor: f32) -> Self {
                self * (1f32 - factor) + end * factor
            }
        }
    };
}

impl_interpolation!(f32);
impl_interpolation!(Vec2);
impl_interpolation!(Vec3);
impl_interpolation!(Vec4);

//----------------------------------------------------------------
