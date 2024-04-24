use serde::{Deserialize, Serialize};

use crate::{flight::Flight, mission_control::MissionControl};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct User {
    pub id: u128,
    pub name: String,
}
impl User {
    pub fn planned_flight(&self, mission_control: &MissionControl) -> Vec<Flight> {
        mission_control.flights
            .iter()
            .filter(|f| f.contains_passenger(self.id))
            .map(|f| f.clone())
            .collect::<Vec<Flight>>()
    }
}
