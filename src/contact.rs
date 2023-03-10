use crate::prelude::*;

#[derive(SystemLabel, Debug, Clone, Eq, PartialEq, Hash)]
pub struct ContactLabel;

//----------------------------------------------------------------

pub struct ContactPlugin;

impl Plugin for ContactPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::Game).with_system(contact_system.label(ContactLabel)),
        );
    }
}

//----------------------------------------------------------------

fn contact_system(
    mut collision_events: EventReader<CollisionEvent>,
    mut ship_asteroid_contact_events: EventWriter<ShipAsteroidContactEvent>,
    mut laser_asteroid_contact_events: EventWriter<LaserAsteroidContactEvent>,
    mut asteroid_guardian_contact_events: EventWriter<AsteroidGuardianContactEvent>,
    ships: Query<&Ship>, lasers: Query<&Laser>, asteroids: Query<&Asteroid>,
    guardians: Query<&Guardian>,
) {
    for event in collision_events.iter() {
        if let CollisionEvent::Started(e1, e2, _flags) = event {
            if ships.get(*e1).is_ok() && asteroids.get(*e2).is_ok() {
                ship_asteroid_contact_events
                    .send(ShipAsteroidContactEvent { ship: *e1, asteroid: *e2 });
            }

            if ships.get(*e2).is_ok() && asteroids.get(*e1).is_ok() {
                ship_asteroid_contact_events
                    .send(ShipAsteroidContactEvent { ship: *e2, asteroid: *e1 });
            }

            if asteroids.get_component::<Asteroid>(*e1).is_ok()
                && lasers.get_component::<Laser>(*e2).is_ok()
            {
                laser_asteroid_contact_events
                    .send(LaserAsteroidContactEvent { laser: *e2, asteroid: *e1 });
            }
            if asteroids.get_component::<Asteroid>(*e2).is_ok()
                && lasers.get_component::<Laser>(*e1).is_ok()
            {
                laser_asteroid_contact_events
                    .send(LaserAsteroidContactEvent { laser: *e1, asteroid: *e2 });
            }

            if guardians.get_component::<Guardian>(*e1).is_ok()
                && asteroids.get_component::<Asteroid>(*e2).is_ok()
            {
                asteroid_guardian_contact_events
                    .send(AsteroidGuardianContactEvent { asteroid: *e2, guardian: *e1 });
            }
            if guardians.get_component::<Guardian>(*e2).is_ok()
                && asteroids.get_component::<Asteroid>(*e1).is_ok()
            {
                asteroid_guardian_contact_events
                    .send(AsteroidGuardianContactEvent { asteroid: *e1, guardian: *e2 });
            }
        }
    }
}
