use std::fmt::{Display, Formatter};
use rocket::serde::{Deserialize, Serialize};

// Wrapper types with private fields

#[derive(Deserialize, Copy, Clone)]
#[serde(crate = "rocket::serde")]
pub struct UnixTimestamp(u64);

#[derive(Deserialize, Copy, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Kelvin(f32);

#[derive(Deserialize, Copy, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Celcius(f32);

#[derive(Deserialize, Copy, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Farenheit(f32);

#[derive(Deserialize, Copy, Clone)]
#[serde(crate = "rocket::serde")]
pub struct HPa(i32);

#[derive(Deserialize, Copy, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Percent(u8);

#[derive(Deserialize, Copy, Clone)]
#[serde(crate = "rocket::serde")]
pub struct UVIndex(f32);

#[derive(Deserialize, Copy, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Metres(f32);

#[derive(Deserialize, Copy, Clone)]
#[serde(crate = "rocket::serde")]
pub struct MetresPerSecond(f32);

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Icon(String);

#[derive(Deserialize, Copy, Clone)]
#[serde(crate = "rocket::serde")]
pub struct WeatherConditionId(u16);

/// degrees (meteorological) -- whatever they are
#[derive(Deserialize, Copy, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Degrees(u16);

#[derive(Deserialize, Copy, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Probability(f32); // might be a percentage

/// Value between 0..1
#[derive(Deserialize, Copy, Clone)]
#[serde(crate = "rocket::serde")]
pub struct MoonPhase(f32);

#[derive(Deserialize, Copy, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Latitude(f32);

#[derive(Deserialize, Copy, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Longitude(f32);

#[derive(Deserialize, Copy, Clone)]
#[serde(crate = "rocket::serde")]
pub struct TimezoneOffset(u32);

// Public structs composed of wrapper types

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct OneCall {
    pub lat: Latitude,
    pub lon: Longitude,
    pub timezone: String,
    pub timezone_offset: TimezoneOffset,
    pub current: CurrentWeather,
    pub daily: Vec<DailyForecast>,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CurrentWeather {
    pub dt: UnixTimestamp,
    pub sunrise: UnixTimestamp,
    pub sunset: UnixTimestamp,
    pub temp: Kelvin,
    pub feels_like: Kelvin,
    pub pressure: HPa,
    pub humidity: Percent,
    pub dew_point: Kelvin,
    pub uvi: UVIndex,
    pub clouds: Percent,
    pub visibility: Metres,
    pub wind_speed: MetresPerSecond,
    pub wind_deg: Degrees,
    pub weather: Vec<Condition>,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Condition {
    pub id: WeatherConditionId,
    pub main: String,
    pub description: String,
    pub icon: Icon,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DailyForecast {
    pub dt: UnixTimestamp,
    pub sunrise: UnixTimestamp,
    pub sunset: UnixTimestamp,
    pub moonrise: UnixTimestamp,
    pub moonset: UnixTimestamp,
    pub moon_phase: MoonPhase,
    pub temp: DayTemp,
    pub feels_like: FeelsLike,
    pub pressure: HPa,
    pub humidity: Percent,
    pub dew_point: Kelvin,
    pub wind_speed: MetresPerSecond,
    pub wind_deg: Degrees,
    pub wind_gust: MetresPerSecond,
    pub weather: Vec<Condition>,
    pub clouds: Percent,
    pub pop: Probability, // probability of precipitation
    pub uvi: UVIndex,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DayTemp {
    pub day: Kelvin,
    pub min: Kelvin,
    pub max: Kelvin,
    pub night: Kelvin,
    pub eve: Kelvin,
    pub morn: Kelvin,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct FeelsLike {
    pub day: Kelvin,
    pub night: Kelvin,
    pub eve: Kelvin,
    pub morn: Kelvin,
}

impl Kelvin {
    pub fn to_celcius(self) -> Celcius {
        Celcius(self.0 - 273.15)
    }

    pub fn to_fahrenheit(self) -> Farenheit {
        Farenheit(self.to_celcius().0 * 1.8 + 32.0)
    }
}

impl Display for Celcius {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.1}°C", self.0)
    }
}

impl Display for Percent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}%", self.0)
    }
}

impl Display for UVIndex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}
