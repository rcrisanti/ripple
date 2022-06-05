use crate::{models::SpotifyToken, schema};
use diesel::{prelude::*, r2d2::ConnectionManager};
use r2d2::PooledConnection;
use rspotify::{clients::BaseClient, scopes, AuthCodeSpotify, Config, Credentials, OAuth, Token};

fn get_creds_oath() -> (Credentials, OAuth) {
    let creds = Credentials::from_env().unwrap();
    let oauth = OAuth::from_env(scopes!(
        "user-read-currently-playing",
        "playlist-modify-private"
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

async fn from_token_refresh(token: Token) -> AuthCodeSpotify {
    let (creds, oauth) = get_creds_oath();
    let spotify = AuthCodeSpotify::new(creds, oauth);
    *spotify.token.lock().await.unwrap() = Some(token);
    spotify
        .refresh_token()
        .await
        .expect("could not refresh user token");
    spotify
}

pub async fn spotify_preconnected(
    username: String,
    connection: PooledConnection<ConnectionManager<PgConnection>>,
) -> Option<AuthCodeSpotify> {
    use schema::spotify_tokens::dsl;

    let spotify_token: Result<SpotifyToken, _> = dsl::spotify_tokens
        .filter(dsl::username.eq(username))
        .get_result(&connection);

    match spotify_token {
        Ok(token) => Some(from_token_refresh(token.into()).await),
        _ => None,
    }
}
