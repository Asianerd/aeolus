use crate::{flight::Flight, mission_control::MissionControl};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Craft {
    pub id: u128,
    pub name: String,
    pub company: u128,
    pub species: Species,
    pub craft_name: String, // Airbus A321
}
impl Craft {
    pub fn current_flight(&self, mission_control: &MissionControl) -> Option<Flight> {
        mission_control.flights
            .iter()
            .filter(|f| f.craft_id == self.id)
            .map(|f| f.clone())
            .next()
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Species {
    Airplane,
    Helicopter,
    Jet
}
