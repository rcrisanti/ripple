use actix_identity::Identity;
use actix_web::{
    web::{self, Data},
    HttpResponse,
};
use diesel::prelude::*;
use rspotify::prelude::*;
use serde::Deserialize;
use tera::{Context, Tera};

use crate::models::NewSpotifyToken;
use crate::{errors::RippleError, schema, spotify, Pool};

#[derive(Debug, Deserialize)]
pub struct SpotifyConnectRequest {
    code: String,
}

pub async fn spotify_connect(
    tera: web::Data<Tera>,
    web::Query(connect_request): web::Query<SpotifyConnectRequest>,
    id: Identity,
    pool: Data<Pool>,
) -> Result<HttpResponse, RippleError> {
    let mut data = Context::new();
    data.insert("title", "spotify connection");

    if let Some(my_username) = id.identity() {
        data.insert("logged_in", "true");
        data.insert("username", &my_username);

        let mut spotify = spotify::init_spotify();

        if spotify.request_token(&connect_request.code).await.is_ok() {
            // use schema::spotify_tokens::dsl::{spotify_tokens, username};

            // let token = spotify
            //     .get_token()
            //     .lock()
            //     .await
            //     .expect("could not obtain token")
            //     .clone()
            //     .expect("token is None");
            // let new_spotify_token = NewSpotifyToken::from_token(my_username, token);

            // let connection = pool.get()?;

            // diesel::insert_into(spotify_tokens)
            //     .values(&new_spotify_token)
            //     .on_conflict(username)
            //     .do_update()
            //     .set(&new_spotify_token)
            //     .execute(&connection)?;

            let connection = pool.get()?;
            spotify::save_token_to_db(&spotify, connection, my_username).await?;

            data.insert("connected_success", "true");
        } else {
            return Ok(HttpResponse::Ok().body("failed to get spotify token"));
        }
    } else {
        return Ok(HttpResponse::Ok().body("not logged in"));
    }

    let rendered = tera.render("spotify_connect.html", &data)?;
    Ok(HttpResponse::Ok().body(rendered))
}
