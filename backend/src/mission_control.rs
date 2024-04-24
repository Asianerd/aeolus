use crate::{airport, country, craft, flight, user};

pub struct MissionControl {
    pub users: Vec<user::User>,

    pub crafts: Vec<craft::Craft>,
    pub flights: Vec<flight::Flight>,
    pub airports: Vec<airport::Airport>,
    pub countries: Vec<country::Country>,
}
impl MissionControl {
    pub fn new() -> MissionControl {
        MissionControl {
            users: vec![],
            crafts: vec![],
            flights: vec![],
            airports: vec![],
            countries: vec![]
        }
    }

    pub fn fetch_craft(&self, id: u128) -> Option<craft::Craft> {
        for c in &self.crafts {
            if c.id == id {
                return Some(c.clone());
            }
        }
        None
    }

    pub fn fetch_flight(&self, id: u128) -> Option<flight::Flight> {
        for f in &self.flights {
            if f.id == id {
                return Some(f.clone());
            }
        }
        None
    }

    pub fn fetch_airport(&self, id: u128) -> Option<airport::Airport> {
        for a in &self.airports {
            if a.id == id {
                return Some(a.clone());
            }
        }
        None
    }

    pub fn fetch_country(&self, id: u128) -> Option<country::Country> {
        for c in &self.countries {
            if c.id == id {
                return Some(c.clone());
            }
        }
        None
    }

    pub fn fetch_user(&self, id:u128) -> Option<user::User> {
        for p in &self.users {
            if p.id == id {
                return Some(p.clone());
            }
        }
        None
    }
}
