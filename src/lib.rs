#[macro_use]
extern crate rocket;

mod config;
pub mod country;
pub mod geocoding;
pub mod home;
pub mod weather;

use std::collections::HashMap;
use std::mem;
use std::sync::Arc;

use celes::Country;
use chrono::{DateTime, Duration, Utc};
use ip2location::DB as GeoDB;
use rocket::tokio::sync::Mutex;

use crate::weather::OneCall;
pub use config::DewpointConfig;

pub type CountryArray = [Country; 250];

pub struct Countries(pub Arc<CountryArray>);
pub struct Ip2Location(pub Arc<Mutex<GeoDB>>);

#[derive(Clone)]
pub struct WeatherCache(Arc<Mutex<HashMap<String, OneCall>>>);

trait Expires {
    const EXPIRATION_MINS: i64 = 10;

    fn stale(&self, now: DateTime<Utc>) -> bool;

    fn fresh(&self, now: DateTime<Utc>) -> bool {
        !self.stale(now)
    }
}

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

    /// Purge expired entries
    pub async fn clean(&self) {
        let mut locked = self.0.lock().await;

        // Move the current map out of the Mutex to filter and replace it
        let cache = mem::replace(&mut *locked, HashMap::new());

        let now = Utc::now();
        *locked = cache
            .into_iter()
            .filter(|(_url, data)| data.fresh(now))
            .collect();
    }

    pub async fn get_or_fetch(&self, url: String) -> Result<OneCall, reqwest::Error> {
        let mut cache = self.0.lock().await;
        match cache.get(&url) {
            Some(data) => {
                let now = Utc::now();
                if data.stale(now) {
                    info!("Weather cache hit: stale");
                    // Stale, refresh cache
                    let data = Self::fetch(&url).await?;
                    cache.insert(url, data.clone());
                    Ok(data)
                } else {
                    info!("Weather cache hit: fresh");
                    // Fresh enough
                    Ok(data.to_owned())
                }
            }
            None => {
                info!("Weather cache miss");
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

impl Expires for OneCall {
    fn stale(&self, now: DateTime<Utc>) -> bool {
        (now - self.current.dt.to_chrono()) > Duration::minutes(Self::EXPIRATION_MINS)
    }
}
