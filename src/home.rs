use std::net::IpAddr;
use std::sync::Arc;

use askama::Template;
use rocket::form::Form;
use rocket::request::FlashMessage;
use rocket::{Route, State};

use dewpoint::weather::OneCall;
use dewpoint::{Countries, CountryArray, DewpointConfig, Ip2Location};

// These are to make the compiler rebuild when they change
// TODO: Check that they don't end up in the final binary
const _HOME: &[u8] = include_bytes!("../templates/home.html");
const _FORECAST: &[u8] = include_bytes!("../templates/forecast.html");

pub fn routes() -> Vec<Route> {
    routes![home, forecast]
}

#[derive(Template)]
#[template(path = "home.html")]
struct HomeContext<'f> {
    title: String,
    ip: String,
    ip_country: String,
    countries: Arc<CountryArray>,
    flash: Option<FlashMessage<'f>>,
}

#[get("/")]
async fn home<'f>(
    client_ip: Option<IpAddr>,
    flash: Option<FlashMessage<'f>>,
    geodb: &State<Ip2Location>,
    countries: &State<Countries>,
) -> HomeContext<'f> {
    let ip_country = client_ip
        .and_then(|ip| {
            let mut geodb = geodb.0.lock().unwrap();
            geodb
                .ip_lookup(ip)
                .ok()
                .and_then(|record| record.country)
                .map(|country| country.short_name)
        })
        .unwrap_or_else(|| String::from("-"));

    HomeContext {
        title: String::from("Home"),
        ip: client_ip.map(|ip| ip.to_string()).unwrap_or_else(|| String::from("Unknown")),
        ip_country,
        countries: Arc::clone(&countries.0),
        flash,
    }
}

#[derive(FromForm)]
struct ForecastForm {
    locality: String,
}

#[derive(Template)]
#[template(path = "forecast.html")]
struct ForecastContext<'f> {
    title: String,
    forecast: OneCall,
    flash: Option<FlashMessage<'f>>,
}

#[post("/", data = "<form>")]
async fn forecast<'f>(
    flash: Option<FlashMessage<'f>>,
    config: &State<DewpointConfig>,
    form: Form<ForecastForm>,
) -> ForecastContext<'f> {
    let lat = "-26.861";
    let lon = "152.957";

    let url = format!("https://api.openweathermap.org/data/2.5/onecall?lat={lat}&lon={lon}&exclude={exclude}&appid={apikey}",
    lat=lat, lon=lon, exclude="minutely,hourly,alerts", apikey=config.openweather_api_key);
    let forecast: OneCall = reqwest::get(url)
        .await
        .expect("FIXME")
        .json()
        .await
        .expect("FIXME");

    ForecastContext {
        title: format!("Forecast for {}", form.locality),
        forecast,
        flash,
    }
}

mod filters {
    use std::env;

    pub fn git_revision(_: &str) -> ::askama::Result<String> {
        Ok(env::var("DEWPOINT_REVISION").unwrap_or_else(|_| String::from("dev")))
    }
}
