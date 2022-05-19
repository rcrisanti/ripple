use actix_identity::Identity;
use actix_web::{web, HttpResponse};
use tera::{Context, Tera};

use crate::errors::RippleError;

pub async fn index(tera: web::Data<Tera>, id: Identity) -> Result<HttpResponse, RippleError> {
    let mut data = Context::new();
    data.insert("title", "home");
    data.insert(
        "logged_in",
        match id.identity() {
            Some(_) => "true",
            None => "false",
        },
    );

    let rendered = tera.render("index.html", &data)?;
    Ok(HttpResponse::Ok().body(rendered))
}
