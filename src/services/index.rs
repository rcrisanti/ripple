use actix_identity::Identity;
use actix_web::{get, web, HttpResponse};
use tera::{Context, Tera};

use crate::errors::RippleError;

#[get("/")]
pub async fn index(tera: web::Data<Tera>, id: Identity) -> Result<HttpResponse, RippleError> {
    let mut data = Context::new();
    data.insert("title", "home");

    if let Some(my_username) = id.identity() {
        log::debug!("logged in");
        data.insert("logged_in", "true");
        data.insert("my_username", &my_username);
    } else {
        log::debug!("not logged in");
        data.insert("logged_in", "false");
    }

    let rendered = tera.render("index.html", &data)?;
    Ok(HttpResponse::Ok().body(rendered))
}
