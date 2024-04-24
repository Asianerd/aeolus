use std::sync::Mutex;

#[macro_use] extern crate rocket;

mod cors;
mod utils;

mod country;
mod user;
mod airport;
mod craft;
mod flight;
mod company;
mod mission_control;

#[get("/")]
fn index() -> String {
    "can you understand me?".to_string()
}

#[get("/")]
fn generate_flights() -> String {
    "".to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::custom(rocket::config::Config::figment().merge(("port", 8000)))
        .manage(Mutex::new(mission_control::MissionControl::new()))
        .mount("/", routes![index])
        .attach(cors::CORS)
}