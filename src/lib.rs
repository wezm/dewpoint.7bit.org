#[macro_use]
extern crate rocket;

mod config;
pub mod country;
pub mod geocoding;
pub mod home;
pub mod weather;

use std::collections::HashMap;
use std::sync::Arc;

use celes::Country;
use chrono::{Duration, Utc};
use ip2location::DB as GeoDB;
use tokio::sync::Mutex;

use crate::weather::OneCall;
pub use config::DewpointConfig;

pub type CountryArray = [Country; 250];

pub struct Countries(pub Arc<CountryArray>);
pub struct Ip2Location(pub Arc<Mutex<GeoDB>>);
pub struct WeatherCache(Arc<Mutex<HashMap<String, OneCall>>>);

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

impl WeatherCache {
    pub fn new() -> Self {
        WeatherCache(Arc::new(Mutex::new(HashMap::new())))
    }

    pub async fn get_or_fetch(&self, url: String) -> Result<OneCall, reqwest::Error> {
        let mut cache = self.0.lock().await;
        match cache.get(&url) {
            Some(data) => {
                let now = Utc::now();
                if (now - data.current.dt.to_chrono()) > Duration::minutes(10) {
                    debug!("weather cache hit is stale");
                    // Stale, refresh cache
                    let data = Self::fetch(&url).await?;
                    cache.insert(url, data.clone());
                    Ok(data)
                } else {
                    debug!("weather cache hit is fresh");
                    // Fresh enough
                    Ok(data.to_owned())
                }
            }
            None => {
                debug!("weather cache miss");
                let data = Self::fetch(&url).await?;
                cache.insert(url, data.clone());
                Ok(data)
            }
        }
    }

    async fn fetch(url: &str) -> Result<OneCall, reqwest::Error> {
        reqwest::get(url).await?.json().await
    }
}
