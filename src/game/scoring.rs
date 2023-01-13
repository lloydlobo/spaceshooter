use crate::prelude::*;

//----------------------------------------------------------------

pub enum Score {
    Guardian(GuardianSize),
    Asteroid(AsteroidSize),
}

impl Score {
    pub const fn score(self) -> u32 {
        match self {
            Self::Guardian(size) => match size {
                GuardianSize::Big => GUARDIAN_SCORE_HIT_POINTS_BIG,
                GuardianSize::Medium => GUARDIAN_SCORE_HIT_POINTS_MEDIUM,
                GuardianSize::Small => GUARDIAN_SCORE_HIT_POINTS_SMALL,
            },
            Self::Asteroid(size) => match size {
                AsteroidSize::Big => ASTEROID_SCORE_HIT_POINTS_BIG,
                AsteroidSize::Medium => ASTEROID_SCORE_HIT_POINTS_MEDIUM,
                AsteroidSize::Small => ASTEROID_SCORE_HIT_POINTS_SMALL,
            },
        }
    }
}

//----------------------------------------------------------------
