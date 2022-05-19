use crate::errors::RippleError;
use crate::models::{LoginUser, User};
use crate::schema;
use crate::Pool;
use actix_identity::Identity;
use actix_web::{web, HttpResponse};
use argonautica::Verifier;
use diesel::prelude::*;
use dotenv::dotenv;
use tera::{Context, Tera};

pub async fn login(tera: web::Data<Tera>, id: Identity) -> Result<HttpResponse, RippleError> {
    let mut data = Context::new();
    data.insert("title", "login");
    data.insert(
        "logged_in",
        match id.identity() {
            Some(_) => "true",
            None => "false",
        },
    );

    let rendered = tera.render("login.html", &data)?;
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn process_login(
    data: web::Form<LoginUser>,
    id: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, RippleError> {
    use schema::users::dsl::{email, users};

    let connection = pool.get()?;
    let user = users
        .filter(email.eq(&data.email))
        .first::<User>(&connection)?;

    dotenv().ok();
    let secret = std::env::var("SECRET_KEY")?;

    let valid = Verifier::default()
        .with_hash(user.password)
        .with_password(data.password.clone())
        .with_secret_key(secret)
        .verify()?;

    if valid {
        let session_token = String::from(user.email);
        id.remember(session_token);
        Ok(HttpResponse::Ok().body(format!("Logged in: {}", data.email)))
    } else {
        Ok(HttpResponse::Ok().body("Password incorrect"))
    }
}
