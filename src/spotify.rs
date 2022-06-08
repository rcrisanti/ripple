use crate::{
    errors::RippleError,
    models::{NewSpotifyToken, SpotifyToken},
    schema,
};
use diesel::{prelude::*, r2d2::ConnectionManager};
use r2d2::PooledConnection;
use rspotify::{clients::BaseClient, scopes, AuthCodeSpotify, Config, Credentials, OAuth, Token};

type Connection = PooledConnection<ConnectionManager<PgConnection>>;

fn get_creds_oath() -> (Credentials, OAuth) {
    let creds = Credentials::from_env().unwrap();
    let oauth = OAuth::from_env(scopes!(
        "user-read-currently-playing",
        "playlist-modify-private",
        "user-read-playback-position",
        "streaming"
    ))
    .unwrap();
    (creds, oauth)
}

pub fn init_spotify() -> AuthCodeSpotify {
    let config = Config {
        token_cached: false,
        token_refreshing: true,
        ..Default::default()
    };
    let (creds, oauth) = get_creds_oath();
    AuthCodeSpotify::with_config(creds, oauth, config)
}

pub async fn save_token_to_db(
    spotify: &AuthCodeSpotify,
    connection: Connection,
    my_username: String,
) -> Result<(), RippleError> {
    use schema::spotify_tokens::dsl::{spotify_tokens, username};

    let token = spotify
        .get_token()
        .lock()
        .await
        .expect("could not obtain token")
        .clone()
        .expect("token is None");
    let new_spotify_token = NewSpotifyToken::from_token(my_username, token);

    diesel::insert_into(spotify_tokens)
        .values(&new_spotify_token)
        .on_conflict(username)
        .do_update()
        .set(&new_spotify_token)
        .execute(&connection)?;

    log::debug!("saved spotify token to database");

    return Ok(());
}

async fn from_token_refresh(
    token: Token,
    connection: Connection,
    my_username: String,
) -> Result<Option<AuthCodeSpotify>, RippleError> {
    let (creds, oauth) = get_creds_oath();
    let spotify = AuthCodeSpotify::new(creds, oauth);
    *spotify.token.lock().await.unwrap() = Some(token);
    match spotify.refresh_token().await {
        Ok(_) => {
            log::debug!("refreshed spotify token");
            save_token_to_db(&spotify, connection, my_username).await?;
            Ok(Some(spotify))
        }
        Err(_) => {
            log::warn!("could not refresh spotify token");
            Ok(None)
        }
    }
}

pub async fn spotify_preconnected(
    username: String,
    connection: Connection,
) -> Result<Option<AuthCodeSpotify>, RippleError> {
    use schema::spotify_tokens::dsl;

    let spotify_token: Result<SpotifyToken, _> = dsl::spotify_tokens
        .filter(dsl::username.eq(&username))
        .get_result(&connection);

    match spotify_token {
        Ok(token) => {
            log::debug!("retrieved spotify token from database");
            match from_token_refresh(token.into(), connection, username).await? {
                Some(spotify) => Ok(Some(spotify)),
                None => Ok(None),
            }
        }
        _ => {
            log::debug!("no spotify token in database for user");
            Ok(None)
        }
    }
}
