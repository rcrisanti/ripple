use actix_identity::Identity;
use actix_web::{
    get,
    web::{self, Data},
    HttpResponse,
};
use rspotify::prelude::*;
use serde::Deserialize;
use tera::{Context, Tera};

use crate::{errors::RippleError, spotify, Pool};

#[derive(Debug, Deserialize)]
pub struct SpotifyConnectRequest {
    code: String,
}

#[get("/spotify-connect")]
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
        log::debug!("initialized spotify auth");

        if spotify.request_token(&connect_request.code).await.is_ok() {
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
