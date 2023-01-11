use crate::prelude::*;

#[derive(SystemLabel, Debug, Clone, Eq, PartialEq, Hash)]
pub struct ContactLabel;

//----------------------------------------------------------------

pub struct ContactPlugin;

impl Plugin for ContactPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(contact_system.label(ContactLabel)),
        );
    }
}

//----------------------------------------------------------------

fn contact_system(
    mut collision_events: EventReader<CollisionEvent>,
    mut ship_asteroid_contact_events: EventWriter<ShipAsteroidContactEvent>,
    mut laser_asteroid_contact_events: EventWriter<LaserAsteroidContactEvent>,
    ships: Query<&Ship>, lasers: Query<&Laser>, asteroids: Query<&Asteroid>,
) {
    todo!()
}
