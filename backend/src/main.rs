use std::sync::Mutex;

#[macro_use] extern crate rocket;

mod cors;
mod utils;
mod log;

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

#[launch]
fn rocket() -> _ {
    rocket::custom(rocket::config::Config::figment().merge(("port", 8000)))
        .manage(Mutex::new(mission_control::MissionControl::load()))
        .mount("/", routes![index])

        .mount("/login", routes![user::login])
        .mount("/signup", routes![user::signup])
        .mount("/fetch_flights", routes![user::fetch_flights])

        .attach(cors::CORS)
}