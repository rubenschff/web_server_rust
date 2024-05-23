use rocket::{Build, config::Config, Rocket};
#[macro_use] extern crate rocket;

mod routes;
pub mod controllers;

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .configure(Config::figment().merge(("port", 5050)))
        .mount("/", routes::get_routes())
}
