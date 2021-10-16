mod home;

#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;
use rocket::{Build, Rocket};
use rocket::fairing::AdHoc;
use rocket_dyn_templates::Template;

use dewpoint::DewpointConfig;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(AdHoc::config::<DewpointConfig>())
        .attach(Template::fairing())
        .mount("/", home::routes())
        .mount("/public", FileServer::from("public"))
}
