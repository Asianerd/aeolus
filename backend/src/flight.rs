use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{mission_control::MissionControl, user::User};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Flight {
    pub id: u128,

    pub craft_id: u128,

    // vector of user ids
    pub passengers: HashMap<u128, PassengerStatus>,

    // times in epoch unix
    pub departure_time: u128,
    pub arrival_time: u128,

    // country (id)
    pub starting: u128,
    pub destination: u128,

    pub state: FlightState
}
impl Flight {
    pub fn international(&self) -> bool {
        // if the flight is to another country
        self.starting != self.destination
    }

    pub fn flight_length(&self) -> u128 {
        // number of seconds
        self.arrival_time - self.departure_time
    }

    pub fn contains_passenger(&self, user_id: u128) -> bool {
        self.passengers.contains_key(&user_id)
    }

    pub fn fetch_passenger(&self, user_id: u128, mission_control: &MissionControl) -> Option<User> {
        if !self.contains_passenger(user_id) {
            return None;
        }
        mission_control.users
            .get(&user_id)
            .map(|u| u.clone())
    }

    pub fn passenger_status(&self, user_id: u128) -> Option<PassengerStatus> {
        if !self.contains_passenger(user_id) {
            return None;
        }
        Some(self.passengers.get(&user_id).unwrap().clone())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum FlightState {
    Delayed,
    Boarding,
    FinalCall,
    Flying,
    Landed
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PassengerStatus {
    Absent,
    Boarded,
    Landed // left the airplane
}
