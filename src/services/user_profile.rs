use crate::models::User;
use crate::schema;
use crate::{errors::RippleError, Pool};
use actix_identity::Identity;
use actix_web::{web, HttpResponse};
use chrono::{DateTime, Local, Utc};
use diesel::prelude::*;
use tera::{Context, Tera};

const DATETIME_FORMAT: &str = "%a %h %d %Y %r %Z";

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
    if let Some(my_username) = id.identity() {
        data.insert("logged_in", "true");
        data.insert("my_username", &my_username);

        if my_username == user.username {
            data.insert("is_my_account", "true");
            data.insert("email", &user.email);
            data.insert(
                "created_at",
                &DateTime::<Utc>::from_utc(user.created_at, Utc)
                    .with_timezone(&Local)
                    .format(DATETIME_FORMAT)
                    .to_string(),
            );
            data.insert(
                "last_login",
                &DateTime::<Utc>::from_utc(user.last_login, Utc)
                    .with_timezone(&Local)
                    .format(DATETIME_FORMAT)
                    .to_string(),
            );
        }
    } else {
        data.insert("logged_in", "false");
    }
    data.insert("requested_username", &user.username);

    let rendered = tera.render("user_profile.html", &data)?;
    Ok(HttpResponse::Ok().body(rendered))
}
