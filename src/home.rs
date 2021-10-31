use std::net::IpAddr;
use std::sync::Arc;

use askama::Template;
use rocket::form::Form;
use rocket::request::FlashMessage;
use rocket::{Route, State};

use crate::country::Country;
use crate::geocoding::Location;
use crate::weather::{OneCall, TemperatureUnit};
use crate::{Countries, CountryArray, DewpointConfig, Ip2Location, WeatherCache};

// These are to make the compiler rebuild when they change
// TODO: Check that they don't end up in the final binary
const _HOME: &[u8] = include_bytes!("../templates/home.html");
const _FORECAST: &[u8] = include_bytes!("../templates/forecast.html");

pub fn routes() -> Vec<Route> {
    routes![home, about, acknowledgements, location, forecast, robots]
}

#[derive(Template)]
#[template(path = "home.html")]
struct HomeContext<'f> {
    title: String,
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
    let ip_country = if let Some(ip) = client_ip {
        let mut geodb = geodb.0.lock().await;
        geodb
            .ip_lookup(ip)
            .ok()
            .and_then(|record| record.country)
            .map(|country| country.short_name)
    } else {
        None
    }
    .unwrap_or_else(|| String::from("-"));

    HomeContext {
        title: String::from("Home"),
        ip_country,
        countries: Arc::clone(&countries.0),
        flash,
    }
}

#[derive(Template)]
#[template(path = "acknowledgements.html")]
struct AcknowlegementsContext<'f> {
    title: String,
    flash: Option<FlashMessage<'f>>,
}

#[get("/acknowledgements")]
fn acknowledgements<'f>(flash: Option<FlashMessage<'f>>) -> AcknowlegementsContext<'f> {
    AcknowlegementsContext {
        title: String::from("Acknowledgements"),
        flash,
    }
}

#[derive(Template)]
#[template(path = "about.html")]
struct AboutContext<'f> {
    title: String,
    flash: Option<FlashMessage<'f>>,
}

#[get("/about")]
fn about<'f>(flash: Option<FlashMessage<'f>>) -> AboutContext<'f> {
    AboutContext {
        title: String::from("About"),
        flash,
    }
}

#[derive(FromForm)]
struct LocationForm {
    locality: String,
    /// ISO 3166-1 alpha-2
    ///
    /// https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2
    country: Country,
}

#[derive(Template)]
#[template(path = "locations.html")]
struct LocationContext<'f> {
    title: String,
    locations: Vec<Location>,
    flash: Option<FlashMessage<'f>>,
}

#[post("/", data = "<form>")]
async fn location<'f>(
    flash: Option<FlashMessage<'f>>,
    config: &State<DewpointConfig>,
    form: Form<LocationForm>,
) -> LocationContext<'f> {
    let url = format!(
        "http://api.openweathermap.org/geo/1.0/direct?q={city},{country}&limit=3&appid={apikey}",
        city = form.locality.trim(),
        country = form.country.code(),
        apikey = config.openweather_api_key
    );
    let locations: Vec<Location> = reqwest::get(url)
        .await
        .expect("FIXME")
        .json()
        .await
        .expect("FIXME");

    LocationContext {
        title: format!("Locations matching {}", form.locality),
        locations,
        flash,
    }
}

#[derive(Template)]
#[template(path = "forecast.html")]
struct ForecastContext<'f> {
    title: String,
    forecast: OneCall,
    unit: TemperatureUnit,
    flash: Option<FlashMessage<'f>>,
}

#[get("/forecast/<country>/<lat>/<lon>/<name>")]
async fn forecast<'f>(
    mut country: String,
    lat: f32,
    lon: f32,
    name: Option<String>,
    flash: Option<FlashMessage<'f>>,
    config: &State<DewpointConfig>,
    weather_cache: &State<WeatherCache>,
) -> ForecastContext<'f> {
    country.make_ascii_uppercase();
    let unit = match country.as_str() {
        // list from https://worldpopulationreview.com/country-rankings/countries-that-use-fahrenheit
        | "BS" // Bahamas
        | "FM" // Micronesia (Federated States of)
        | "KY" // Cayman Islands
        | "LR" // Liberia
        | "MH" // Marshall Islands
        | "PW" // Palau
        | "US" // United States
        => TemperatureUnit::Fahrenheit,
        _ => TemperatureUnit::Celsius,
    };

    let url = format!("https://api.openweathermap.org/data/2.5/onecall?lat={lat}&lon={lon}&exclude={exclude}&appid={apikey}",
    lat=lat, lon=lon, exclude="minutely,hourly,alerts", apikey=config.openweather_api_key);
    let forecast = weather_cache
        .get_or_fetch(url)
        .await
        .expect("FIXME weather error");

    ForecastContext {
        title: format!(
            "Forecast for {}",
            name.unwrap_or_else(|| String::from("Unknown"))
        ),
        forecast,
        unit,
        flash,
    }
}

#[get("/robots.txt")]
fn robots() -> &'static str {
    "User-agent: *\nDisallow: /forecast/"
}

mod filters {
    use super::rocket_uri_macro_about;
    use super::rocket_uri_macro_acknowledgements;
    use super::rocket_uri_macro_home;
    use std::{env, fmt};

    pub fn git_revision(_: &str) -> ::askama::Result<String> {
        Ok(env::var("DEWPOINT_REVISION").unwrap_or_else(|_| String::from("dev")))
    }

    pub fn url(name: &str) -> ::askama::Result<String> {
        match name {
            "home" => Ok(uri!(home()).to_string()),
            "about" => Ok(uri!(about()).to_string()),
            "acknowledgements" => Ok(uri!(acknowledgements()).to_string()),
            _ => Err(askama::Error::Fmt(fmt::Error)),
        }
    }
}
