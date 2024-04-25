use std::collections::HashMap;

use crate::{airport, country, craft, flight, user};

pub struct MissionControl {
    pub users: HashMap<u128, user::User>,

    pub crafts: HashMap<u128, craft::Craft>,
    pub flights: HashMap<u128, flight::Flight>,
    pub airports: HashMap<u128, airport::Airport>,
    pub countries: HashMap<u128, country::Country>,
}
impl MissionControl {
    pub fn new() -> MissionControl {
        MissionControl {
            users: HashMap::new(),
            crafts: HashMap::new(),
            flights: HashMap::new(),
            airports: HashMap::new(),
            countries: HashMap::new()
        }
    }

    pub fn save(&self) {
        user::User::save(&self);
    }

    pub fn load() -> MissionControl {
        MissionControl {
            users: user::User::load(),
            crafts: HashMap::new(),
            flights: HashMap::new(),
            airports: HashMap::new(),
            countries: HashMap::new()
        }
    }
}
