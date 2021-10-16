use rocket::request::FlashMessage;
use rocket::response::{Debug, Flash, Redirect};
use rocket::serde::Serialize;
use rocket::Route;
use rocket_dyn_templates::Template;
use std::convert::Infallible;

pub fn routes() -> Vec<Route> {
    routes![
        home,
    ]
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct HomeContext<'f> {
    title: String,
    flash: Option<FlashMessage<'f>>,
}

#[get("/")]
pub async fn home(
    flash: Option<FlashMessage<'_>>,
) -> Result<Template, Debug<Infallible>> {
    Ok(Template::render(
        "home",
        HomeContext {
            title: String::from("Home"),
            flash,
        },
    ))
}

