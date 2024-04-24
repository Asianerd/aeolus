use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Airport {
    pub id: u128,
    pub name: String, // Kuala Lumpur International Airport
    pub shortened: String, // KLIA
    pub gates: Vec<String>, // J1, J2, A, B, etc
    pub country_id: u128,
    pub abroad: bool // if can fly overseas
}
