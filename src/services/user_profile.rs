use crate::models::User;
use crate::schema;
use crate::{errors::RippleError, Pool};
use actix_identity::Identity;
use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use tera::{Context, Tera};

pub async fn user_profile(
    tera: web::Data<Tera>,
    web::Path(requested_user): web::Path<String>,
    pool: web::Data<Pool>,
    id: Identity,
) -> Result<HttpResponse, RippleError> {
    use schema::users::dsl::{username, users};
    let connection = pool.get()?;

    let user: User = users
        .filter(username.eq(requested_user))
        .get_result(&connection)?;

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
