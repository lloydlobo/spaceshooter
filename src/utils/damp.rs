use crate::prelude::*;

//----------------------------------------------------------------

pub trait Damp {
    fn damp(self, target: Self, speed: f32, delta_seconds: f32) -> Self;
}

impl<T> Damp for T
where
    T: Interpolation,
{
    fn damp(self, target: Self, speed: f32, delta_seconds: f32) -> Self {
        self.lerp(target, 1f32 - (speed.neg() * delta_seconds).exp())
    }
}

//----------------------------------------------------------------
