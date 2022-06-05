use actix_identity::Identity;
use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use tera::{Context, Tera};

use crate::errors::RippleError;
use crate::models::{SpotifyToken, User};
use crate::{schema, spotify::spotify_preconnected, Pool};

pub async fn home(
    tera: web::Data<Tera>,
    id: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, RippleError> {
    use schema::spotify_tokens::dsl::{spotify_tokens, username};
    let mut data = Context::new();
    data.insert("title", "home");

    if let Some(my_username) = id.identity() {
        data.insert("my_username", &my_username);
        data.insert("logged_in", "true");

        // let connection = pool.get()?;

        // let spotify_token: Result<SpotifyToken, _> = spotify_tokens
        //     .filter(username.eq(my_username))
        //     .get_result(&connection);

        // if let Ok(token) = spotify_token {
        //     let spotify = from_token_refresh(token.into()).await;
        //     data.insert("connected_to_spotify", "true");
        // }

        let connection = pool.get()?;
        let spotify_auth = spotify_preconnected(my_username, connection).await;

        if let Some(spotify) = spotify_auth {
            data.insert("connected_to_spotify", "true");
        }
    } else {
        return Ok(HttpResponse::Ok().body("not logged in"));
    }

    let rendered = tera.render("home.html", &data)?;
    Ok(HttpResponse::Ok().body(rendered))
}
