use crate::errors::RippleError;
use crate::models::{LoginUser, User};
use crate::schema;
use crate::Pool;
use actix_identity::Identity;
use actix_web::{get, post, web, HttpResponse};
use argonautica::Verifier;
use diesel::prelude::*;
use dotenv::dotenv;
use tera::{Context, Tera};

#[get("/login")]
pub async fn login(tera: web::Data<Tera>, id: Identity) -> Result<HttpResponse, RippleError> {
    let mut data = Context::new();
    data.insert("title", "login");
    if let Some(my_username) = id.identity() {
        log::debug!("logged in");
        data.insert("logged_in", "true");
        data.insert("my_username", &my_username);
    } else {
        log::debug!("not logged in");
        data.insert("logged_in", "false");
    }

    let rendered = tera.render("login.html", &data)?;
    Ok(HttpResponse::Ok().body(rendered))
}

#[post("/login")]
pub async fn process_login(
    data: web::Form<LoginUser>,
    id: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, RippleError> {
    use schema::users::dsl::{last_login, username, users};

    let connection = pool.get()?;
    let user = users
        .filter(username.eq(&data.username))
        .first::<User>(&connection)?;
    log::debug!("retrieved user from database");

    dotenv().ok();
    let secret = std::env::var("SECRET_KEY")?;

    let valid = Verifier::default()
        .with_hash(user.password)
        .with_password(data.password.clone())
        .with_secret_key(secret)
        .verify()?;

    if valid {
        log::debug!("correct password");

        // update last_login field
        diesel::update(users)
            .filter(username.eq(&user.username))
            .set(last_login.eq(chrono::Local::now().naive_utc()))
            .execute(&connection)?;
        log::debug!("updated login time in database");

        let session_token = String::from(user.username);
        id.remember(session_token);
        log::debug!("remembered session token");

        Ok(HttpResponse::Ok().body(format!("Logged in: {}", data.username)))
    } else {
        log::debug!("incorrect password");
        Ok(HttpResponse::Ok().body("Password incorrect"))
    }
}
