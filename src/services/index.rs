use actix_identity::Identity;
use actix_web::{web, HttpResponse};
use tera::{Context, Tera};

use crate::errors::RippleError;

pub async fn index(tera: web::Data<Tera>, id: Identity) -> Result<HttpResponse, RippleError> {
    let mut data = Context::new();
    data.insert("title", "home");

    if let Some(my_username) = id.identity() {
        data.insert("logged_in", "true");
        data.insert("my_username", &my_username);
    } else {
        data.insert("logged_in", "false");
    }

    let rendered = tera.render("index.html", &data)?;
    Ok(HttpResponse::Ok().body(rendered))
}
