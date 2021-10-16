use rocket::serde::Deserialize;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DewpointConfig {
    pub openweather_api_key: String,
}
