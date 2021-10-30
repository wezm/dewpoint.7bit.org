//! OpenWeather Geo-coding API
//!
//! https://openweathermap.org/api/geocoding-api

use rocket::http::uri::Origin;
use rocket::serde::Deserialize;
use rocket::uri;

use crate::country::{country_from_code, Country};
use crate::home::rocket_uri_macro_forecast;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Location {
    /// Name of the found location
    pub name: String,
    /// Geographical coordinates of the found location (latitude)
    pub lat: f32,
    /// Geographical coordinates of the found location (longitude)
    pub lon: f32,
    /// Country of the found location
    #[serde(deserialize_with = "country_from_code")]
    pub country: Country,
}

impl Location {
    pub fn url(&self) -> Origin {
        uri!(forecast(
            self.country.code().to_ascii_lowercase(),
            self.lat,
            self.lon,
            &self.name
        ))
    }
}
