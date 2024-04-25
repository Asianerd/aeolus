use serde::{Deserialize, Serialize};

use crate::{airport::Airport, mission_control::MissionControl};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Country {
    pub id: u128,
    pub name: String,
    pub shortened: String,
}
impl Country {
    pub fn fetch_airports(&self, control: &MissionControl) -> Vec<Airport> {
        // all airports in the country
        control.airports
            .clone()
            .iter()
            .filter(|(_, a)| a.country_id == self.id)
            .map(|(_, a)| a.clone())
            .collect::<Vec<Airport>>()
    }

    pub fn fetch_abroad_airports(&self, control: &MissionControl) -> Vec<Airport> {
        // all airports that can fly out the country
        control.airports
            .clone()
            .iter()
            .filter(|(_, a)| (a.country_id == self.id) && a.abroad)
            .map(|(_, a)| a.clone())
            .collect::<Vec<Airport>>()
    }
}
