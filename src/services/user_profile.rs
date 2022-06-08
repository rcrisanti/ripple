use crate::models::User;
use crate::schema;
use crate::{errors::RippleError, Pool};
use actix_identity::Identity;
use actix_web::{get, web, HttpResponse};
use diesel::prelude::*;
use tera::{Context, Tera};

#[get("/user/{username}")]
pub async fn user_profile(
    tera: web::Data<Tera>,
    requested_user: web::Path<String>,
    pool: web::Data<Pool>,
    id: Identity,
) -> Result<HttpResponse, RippleError> {
    use schema::users::dsl::{username, users};
    let connection = pool.get()?;

    let requested_user = requested_user.into_inner();

    let user: User = users
        .filter(username.eq(&requested_user))
        .get_result(&connection)?;

    log::debug!("retrieved user {} from database", &requested_user);

    let mut data = Context::new();
    data.insert("title", &user.username);
    data.insert(
        "logged_in",
        match id.identity() {
            Some(_) => "true",
            None => "false",
        },
    );
    data.insert("username", &user.username);

    let rendered = tera.render("user_profile.html", &data)?;
    Ok(HttpResponse::Ok().body(rendered))
}
