//! OpenWeather OneCall deserialization
//!
//! https://openweathermap.org/api/one-call-api

use std::fmt::{Display, Formatter};

use chrono::{DateTime, FixedOffset, TimeZone, Utc};
use rocket::serde::Deserialize;

// Wrapper types with private fields

#[derive(Deserialize, Copy, Clone)]
#[serde(crate = "rocket::serde")]
pub struct UnixTimestamp(i64);

#[derive(Deserialize, Copy, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Kelvin(f32);

#[derive(Deserialize, Copy, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Celsius(f32);

#[derive(Deserialize, Copy, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Fahrenheit(f32);

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

#[derive(Deserialize, Clone)]
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
pub struct TimezoneOffset(i32);

// Public structs composed of wrapper types

#[derive(Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct OneCall {
    pub lat: Latitude,
    pub lon: Longitude,
    pub timezone: String,
    pub timezone_offset: TimezoneOffset,
    pub current: CurrentWeather,
    pub daily: Vec<DailyForecast>,
}

#[derive(Deserialize, Clone)]
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

#[derive(Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Condition {
    pub id: WeatherConditionId,
    pub main: String,
    pub description: String,
    pub icon: Icon,
}

#[derive(Deserialize, Clone)]
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

#[derive(Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct DayTemp {
    pub day: Kelvin,
    pub min: Kelvin,
    pub max: Kelvin,
    pub night: Kelvin,
    pub eve: Kelvin,
    pub morn: Kelvin,
}

#[derive(Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct FeelsLike {
    pub day: Kelvin,
    pub night: Kelvin,
    pub eve: Kelvin,
    pub morn: Kelvin,
}

#[derive(Copy, Clone, Debug)]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
}

impl OneCall {
    pub fn sunrise(&self) -> String {
        self.current.sunrise.time_12h(&self.timezone_offset)
    }

    pub fn sunset(&self) -> String {
        self.current.sunset.time_12h(&self.timezone_offset)
    }

    pub fn current_temp(&self, unit: &TemperatureUnit) -> String {
        match unit {
            TemperatureUnit::Celsius => self.current.temp.to_celcius().to_string(),
            TemperatureUnit::Fahrenheit => self.current.temp.to_fahrenheit().to_string(),
        }
    }

    pub fn feels_like(&self, unit: &TemperatureUnit) -> String {
        match unit {
            TemperatureUnit::Celsius => self.current.feels_like.to_celcius().to_string(),
            TemperatureUnit::Fahrenheit => self.current.feels_like.to_fahrenheit().to_string(),
        }
    }

    pub fn dew_point(&self, unit: &TemperatureUnit) -> String {
        match unit {
            TemperatureUnit::Celsius => self.current.dew_point.to_celcius().to_string(),
            TemperatureUnit::Fahrenheit => self.current.dew_point.to_fahrenheit().to_string(),
        }
    }
}

impl DailyForecast {
    pub fn dew_point(&self, unit: &TemperatureUnit) -> String {
        match unit {
            TemperatureUnit::Celsius => self.dew_point.to_celcius().to_string(),
            TemperatureUnit::Fahrenheit => self.dew_point.to_fahrenheit().to_string(),
        }
    }

    pub fn max(&self, unit: &TemperatureUnit) -> String {
        match unit {
            TemperatureUnit::Celsius => self.temp.max.to_celcius().to_string(),
            TemperatureUnit::Fahrenheit => self.temp.max.to_fahrenheit().to_string(),
        }
    }

    pub fn sunrise(&self, timezone_offset: &TimezoneOffset) -> String {
        self.sunrise.time_12h(timezone_offset)
    }

    pub fn sunset(&self, timezone_offset: &TimezoneOffset) -> String {
        self.sunset.time_12h(timezone_offset)
    }
}

impl Kelvin {
    pub fn to_celcius(self) -> Celsius {
        Celsius(self.0 - 273.15)
    }

    pub fn to_fahrenheit(self) -> Fahrenheit {
        Fahrenheit(self.to_celcius().0 * 1.8 + 32.0)
    }
}

impl UnixTimestamp {
    pub fn day_date(self, tz_offset: &TimezoneOffset) -> String {
        self.in_timezone(*tz_offset)
            .format("%A, %-d %B")
            .to_string()
    }

    pub fn time_12h(self, tz_offset: &TimezoneOffset) -> String {
        self.in_timezone(*tz_offset).format("%I:%M %p").to_string()
    }

    pub fn to_chrono(self) -> DateTime<Utc> {
        Utc.timestamp(self.0, 0)
    }

    fn in_timezone(self, tz_offset: TimezoneOffset) -> DateTime<FixedOffset> {
        self.to_chrono()
            .with_timezone(&FixedOffset::east(tz_offset.0))
    }
}

impl Display for Celsius {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.1}°C", self.0)
    }
}

impl Display for Fahrenheit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.1}°F", self.0)
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
