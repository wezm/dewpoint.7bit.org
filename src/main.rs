mod home;

#[macro_use]
extern crate rocket;

use rocket::fairing::AdHoc;
use rocket::fs::FileServer;

use dewpoint::DewpointConfig;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(AdHoc::config::<DewpointConfig>())
        .mount("/", home::routes())
        .mount("/public", FileServer::from("public"))
}
