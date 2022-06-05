use actix_identity::Identity;
use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use rspotify::clients::OAuthClient;
use rspotify::model::PlayableItem;
use tera::{Context, Tera};

use crate::errors::RippleError;
use crate::models::{SpotifyToken, User};
use crate::{schema, spotify::spotify_preconnected, Pool};

pub async fn home(
    tera: web::Data<Tera>,
    id: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, RippleError> {
    let mut data = Context::new();
    data.insert("title", "home");

    if let Some(my_username) = id.identity() {
        data.insert("my_username", &my_username);
        data.insert("logged_in", "true");

        let connection = pool.get()?;
        let spotify_auth = spotify_preconnected(my_username, connection).await;

        if let Some(spotify) = spotify_auth {
            data.insert("connected_to_spotify", "true");

            let context = spotify.current_user_playing_item().await?;

            match context {
                Some(context) => {
                    let item = context.item.as_ref().unwrap();
                    match item {
                        PlayableItem::Track(track) => {
                            dbg!(track);
                            data.insert("currently_playing_name", &track.name);
                            data.insert(
                                "currently_playing_artist",
                                &track.artists.first().expect("N/A").name,
                            );
                            let imgs = track
                                .album
                                .images
                                .clone()
                                .into_iter()
                                .map(|img| {
                                    format!(
                                        "{url} {width}w",
                                        url = img.url,
                                        width = img.width.unwrap_or(1)
                                    )
                                })
                                .collect::<Vec<String>>()
                                .join(", ");
                            log::debug!("images: {}", imgs);
                            data.insert("img_srcset", &imgs);
                            // data.insert(
                            //     "img_src",
                            //     &track.album.images.first().expect("no album images").url,
                            // );
                        }
                        PlayableItem::Episode(episode) => todo!(),
                    };
                }
                None => todo!(),
            };
        }
    } else {
        return Ok(HttpResponse::Ok().body("not logged in"));
    }

    let rendered = tera.render("home.html", &data)?;
    Ok(HttpResponse::Ok().body(rendered))
}
