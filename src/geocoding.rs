//! OpenWeather Geo-coding API
//!
//! https://openweathermap.org/api/geocoding-api

pub struct Location {
    /// Name of the found location
    name: String,
    /// Geographical coordinates of the found location (latitude)
    lat: f32,
    /// Geographical coordinates of the found location (longitude)
    lon: f32,
    /// Country of the found location
    country: CountryCode,
}

struct CountryCode {}
