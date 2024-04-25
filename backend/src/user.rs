use std::collections::HashMap;
use std::fs;
use std::sync::Mutex;

use rocket::request::Request;
use rocket::data::{self, Data, FromData};
use rocket::outcome::Outcome;
use rocket::State;
use serde::{Deserialize, Serialize};

use crate::utils;
use crate::{flight::Flight, mission_control::MissionControl};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct User {
    pub id: u128,
    pub username: String,
}
impl User {
    pub fn save(mission_control: &MissionControl) {
        fs::write("data/users.json", serde_json::to_string_pretty(&mission_control.users).unwrap()).unwrap();
    }

    pub fn load() -> HashMap<u128, User> {
        serde_json::from_str(fs::read_to_string("data/users.json").unwrap().as_str()).unwrap()
    }

    pub fn username_exists(mission_control: &MissionControl, username: &String) -> bool {
        for (_, user) in &mission_control.users {
            if user.username == *username {
                return true;
            } 
        }
        false
    }

    pub fn generate_user_id(mission_control: &MissionControl) -> u128 {
        mission_control.users
            .keys()
            .max()
            .map_or(0, |i| i + 1)
    }

    pub fn lookup_user_id(mission_control: &MissionControl, username: &String) -> Option<u128> {
        if !User::username_exists(mission_control, username) {
            return None;
        }
        mission_control.users
            .iter()
            .filter(|(_, u)| u.username == *username)
            .map(|(id, _)| *id)
            .next()
    }

    pub fn fetch_flights(&self, mission_control: &MissionControl) -> Vec<Flight> {
        mission_control.flights
            .iter()
            .filter(|(_, f)| f.contains_passenger(self.id))
            .map(|(_, f)| f.clone())
            .collect::<Vec<Flight>>()
    }
}



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginInformation {
    pub username: String,
    pub password: String
}
impl LoginInformation {
    // handles anything to do with password or logging in
    fn get_passwords() -> HashMap<u128, String> {
        serde_json::from_str(fs::read_to_string("data/passwords.json").unwrap().as_str()).unwrap()
    }

    fn add_password(user_id: u128, password: &String) {
        let mut p = LoginInformation::get_passwords();
        p.insert(user_id, password.clone());
        fs::write("data/passwords.json", serde_json::to_string_pretty(&p).unwrap()).unwrap();
    }

    pub fn login(&self, mission_control: &MissionControl) -> LoginResult {
        match User::lookup_user_id(mission_control, &self.username) {
            Some(id) => match LoginInformation::get_passwords().get(&id) {
                Some(p) => if self.password == *p { LoginResult::Success(id) } else { LoginResult::PasswordWrong },
                None => LoginResult::PasswordNoExist
            },
            None => LoginResult::UsernameNoExist
        }
    }

    pub fn signup(&self, mission_control: &mut MissionControl) -> LoginResult {
        if User::username_exists(mission_control, &self.username) {
            return LoginResult::UsernameTaken;
        }

        let id = User::generate_user_id(mission_control);
        mission_control.users.insert(id, User {
            id,
            username: self.username.clone()
        });
        LoginInformation::add_password(id, &self.password);
        mission_control.save();

        LoginResult::Success(id)
    }
}

#[rocket::async_trait]
impl<'l> FromData<'l> for LoginInformation {
    type Error = LoginInfoParseError;

    async fn from_data(_req: &'l Request<'_>, mut data: Data<'l>) -> data::Outcome<'l, Self> {
        let result = data.peek(512).await.to_vec();
        
        let result = result.iter().map(|x| (x.clone()) as char).collect::<String>();

        let result: HashMap<String, String> = serde_json::from_str(result.as_str()).unwrap();

        Outcome::Success(LoginInformation {
            username: result.get("username").unwrap().clone(),
            password: result.get("password").unwrap().clone()
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum LoginInfoParseError {
    Success,

    ParsingError
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum LoginResult {
    Success(u128),
    UsernameNoExist,
    PasswordNoExist, // consistency issue

    PasswordWrong,

    UsernameTaken
}



// #region api calls
#[post("/", data="<login>")]
pub fn login(db: &State<Mutex<MissionControl>>, login: LoginInformation) -> String {
    let db = db.lock().unwrap();
    serde_json::to_string(&login.login(&db)).unwrap()
}

#[post("/", data="<login>")]
pub fn signup(db: &State<Mutex<MissionControl>>, login: LoginInformation) -> String {
    let mut db = db.lock().unwrap();
    serde_json::to_string(&login.signup(&mut db)).unwrap()
}

#[post("/", data="<login>")]
pub fn fetch_flights(db: &State<Mutex<MissionControl>>, login: LoginInformation) -> String {
    let db = db.lock().unwrap();
    let result = login.login(&db);
    match result {
        LoginResult::Success(id) => utils::parse_response(Ok(serde_json::to_string(&db.users.get(&id).unwrap().fetch_flights(&db)).unwrap())),
        _ => utils::parse_response(Err(serde_json::to_string(&result).unwrap()))
    }

}
// #endregion
