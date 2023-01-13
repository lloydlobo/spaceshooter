use crate::prelude::*;

//state.rs
//----------------------------------------------------------------

/// `Component` to tag an entity as only needed in some of the states
#[derive(Debug, Component)]
pub struct ForState<T> {
    pub states: Vec<T>,
}

//player_ship.rs
//----------------------------------------------------------------

/// The ship used by a `Player`.
#[derive(Component)]
pub struct Ship {
    /// Ship rotation speed in `rad/s`.
    pub rotation_speed: f32,
    /// Ship thrust N (Newton).
    pub thrust: f32,
    /// Ship life health points.
    pub life: u32,
    /// Cannon auto-fire timer.
    pub cannon_timer: Timer,
    /// Id of the controlling player. `Player` 1 or `Player` 2.
    pub player_id: u32,
    /// Timer triggered after being hit that provides short-term invincibility.
    pub invincible_timer: Timer,
    /// Total duration of invincibility, accumulating when renewed.
    pub invincible_time_secs: f32,
}

/// Player ship damage.
#[derive(Component, Clone, Copy)]
pub struct Damage {
    pub value: u32,
}

//explosion.rs
//----------------------------------------------------------------

#[derive(Component)]
pub struct Explosion {
    pub timer: Timer,
    pub start_scale: f32,
    pub end_scale: f32,
}

//hud.rs
//----------------------------------------------------------------

#[derive(Component)]
pub struct UiScore {}
#[derive(Component)]
pub struct UiLife {
    pub min: u32,
}

//laser.rs
//----------------------------------------------------------------

#[derive(Component)]
pub struct Laser {
    pub despawn_timer: Timer,
}

//menu.rs
//----------------------------------------------------------------

#[derive(Component)]
pub struct DrawBlinkTimer(pub Timer);

//guardians/formation.rs
//----------------------------------------------------------------

/// Component - Guardian Formation (per guardian).
#[derive(Component, Clone)]
pub struct GuardianFormation {
    /// Start at x/y.
    pub start: (f32, f32),
    /// Radius of circular/elliptical formation.
    pub radius: (f32, f32),
    /// Pivot along x/y.
    pub pivot: (f32, f32),
    /// Speed of members moving in formation.
    pub speed: f32,
    /// Start angle.
    pub angle: f32, // Change pre tick.
}
