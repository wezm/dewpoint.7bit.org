//! OpenWeather Geo-coding API
//!
//! https://openweathermap.org/api/geocoding-api

use rocket::http::uri::Origin;
use rocket::serde::Deserialize;
use rocket::uri;
use std::collections::HashMap;

use crate::country::{country_from_code, Country};
use crate::home::rocket_uri_macro_forecast;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Location {
    /// Name of the found location
    name: String,
    /// Geographical coordinates of the found location (latitude)
    pub lat: f32,
    /// Geographical coordinates of the found location (longitude)
    pub lon: f32,
    /// State
    state: Option<String>,
    /// Country of the found location
    #[serde(deserialize_with = "country_from_code")]
    pub country: Country,
    /// Translated names
    #[serde(default)]
    local_names: HashMap<String, String>,
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

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn state(&self) -> Option<&str> {
        self.state.as_deref()
    }

    pub fn country_name(&self) -> &str {
        self.country.0.long_name
    }
}
