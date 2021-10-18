mod home;

#[macro_use]
extern crate rocket;

use ip2location::DB as GeoDB;
use rocket::fairing::AdHoc;
use rocket::fs::FileServer;

use dewpoint::{Countries, DewpointConfig, Ip2Location};

#[launch]
fn rocket() -> _ {
    let geodb = Ip2Location::new(
        GeoDB::from_file("IP2LOCATION-LITE-DB1.BIN").expect("FIXME: unable to open geo ip db"),
    );
    let countries = Countries::new();

    rocket::build()
        .manage(geodb)
        .manage(countries)
        .attach(AdHoc::config::<DewpointConfig>())
        .mount("/", home::routes())
        .mount("/public", FileServer::from("public"))
}
