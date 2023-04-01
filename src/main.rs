#[macro_use]
extern crate rocket;

use ip2location::LocationDB as GeoDB;
use rocket::fairing::AdHoc;
use rocket::fs::FileServer;
use rocket::tokio::select;
use rocket::tokio::time::{self, Duration, Instant, MissedTickBehavior};

use dewpoint::{home, WeatherCache};
use dewpoint::{Countries, DewpointConfig, Ip2Location};

const CLEAN_PERIOD: u64 = 4 * 60 * 60; // 4 hours (in seconds)

#[launch]
fn rocket() -> _ {
    let geodb = Ip2Location::new(
        GeoDB::from_file("IP2LOCATION-LITE-DB1.BIN").expect("FIXME: unable to open geo ip db"),
    );
    let countries = Countries::new();
    let weather_cache = WeatherCache::new();

    rocket::build()
        .manage(geodb)
        .manage(countries)
        .manage(weather_cache.clone())
        .attach(AdHoc::config::<DewpointConfig>())
        .attach(cache_cleaner(weather_cache))
        .mount("/", home::routes())
        .mount("/public", FileServer::from("public"))
}

fn cache_cleaner(cache: WeatherCache) -> AdHoc {
    AdHoc::on_liftoff("Cache cleaner", |rocket| {
        Box::pin(async move {
            let mut shutdown = rocket.shutdown();
            rocket::tokio::spawn(async move {
                let period = Duration::from_secs(CLEAN_PERIOD);
                let start = Instant::now() + period;
                let mut interval = time::interval_at(start, period);
                // schedule the next tick `period` from whenever the last tick occurs.
                interval.set_missed_tick_behavior(MissedTickBehavior::Delay);
                loop {
                    select! {
                        _ = interval.tick() => {
                            info!("Cleaning weather cache");
                            cache.clean().await;
                            info!("Weather cache cleaned");
                        },
                        _ = &mut shutdown => break,
                    };
                }
            });
        })
    })
}
