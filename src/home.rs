use rocket::request::FlashMessage;

use askama::Template;
use rocket::form::Form;
use rocket::{Route, State};

use dewpoint::openweather::OneCall;
use dewpoint::DewpointConfig;

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
    flash: Option<FlashMessage<'f>>,
}

#[get("/")]
async fn home<'f>(flash: Option<FlashMessage<'f>>) -> HomeContext<'f> {
    HomeContext {
        title: String::from("Home"),
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
