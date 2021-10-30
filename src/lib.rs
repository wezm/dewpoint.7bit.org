#[macro_use]
extern crate rocket;

mod config;
pub mod country;
pub mod geocoding;
pub mod home;
pub mod weather;

use celes::Country;
use ip2location::DB as GeoDB;
use std::sync::{Arc, Mutex};

pub use config::DewpointConfig;

pub type CountryArray = [Country; 250];

pub struct Countries(pub Arc<CountryArray>);
pub struct Ip2Location(pub Arc<Mutex<GeoDB>>);

impl Countries {
    pub fn new() -> Self {
        Countries(Arc::new(Country::get_countries()))
    }
}

impl Ip2Location {
    pub fn new(geodb: GeoDB) -> Self {
        Ip2Location(Arc::new(Mutex::new(geodb)))
    }
}
