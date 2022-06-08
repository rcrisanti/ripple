use crate::models::User;
use crate::schema;
use crate::{errors::RippleError, spotify, Pool};
use actix_identity::Identity;
use actix_web::{get, web, HttpResponse};
use chrono::{DateTime, Local, Utc};
use diesel::prelude::*;
use tera::{Context, Tera};

const DATETIME_FORMAT: &str = "%a %h %d %Y %r %Z";

#[get("/account")]
pub async fn account(
    tera: web::Data<Tera>,
    pool: web::Data<Pool>,
    id: Identity,
) -> Result<HttpResponse, RippleError> {
    use schema::users::dsl::{username, users};
    let connection = pool.get()?;

    let mut data = Context::new();
    data.insert("title", "account");
    if let Some(my_username) = id.identity() {
        let user: User = users
            .filter(username.eq(&my_username))
            .get_result(&connection)?;
        log::debug!("retrieved user from database");

        data.insert("logged_in", "true");
        data.insert("my_username", &my_username);
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

        // setup spotify auth
        match spotify::spotify_preconnected(my_username, connection).await? {
            Some(_) => data.insert("spotify_connected", "true"),
            None => {
                let spotify = spotify::init_spotify();
                let url = spotify.get_authorize_url(true).unwrap();

                data.insert("spotify_auth_url", &url);
            }
        }
    } else {
        return Ok(HttpResponse::Ok().body("not logged in"));
    }

    let rendered = tera.render("account.html", &data)?;
    Ok(HttpResponse::Ok().body(rendered))
}
