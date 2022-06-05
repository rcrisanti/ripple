use std::collections::HashSet;

use super::errors::RippleError;
use crate::schema::{spotify_tokens, users};

use argonautica::Hasher;
use chrono::{DateTime, Duration, Local, NaiveDateTime, Utc};
use diesel::{AsChangeset, Identifiable, Insertable, Queryable};
use dotenv::dotenv;
use regex::Regex;
use rspotify::Token;
use serde::{Deserialize, Serialize};
use validator::Validate;

lazy_static! {
    static ref PASSWORD_REGEX: Regex = Regex::new(r"[a-zA-Z\d@#$%^&-+=()!? ]{8,24}$").unwrap();
    static ref USERNAME_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9]{4,18}$").unwrap();
}

#[derive(Debug, Identifiable, Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub last_login: NaiveDateTime,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UserForm {
    #[validate(
        length(min = 4, max = 18, message = "username should be 4-18 characters"),
        regex(
            path = "USERNAME_REGEX",
            message = "username should be only made up of letters, numbers, and digits"
        )
    )]
    pub username: String,
    #[validate(email(message = "please enter a valid email"))]
    pub email: String,
    #[validate(
        length(min = 8, max = 24, message = "password should be 8-24 characters"),
        regex(
            path = "PASSWORD_REGEX",
            message = "password should be made up of letters, numbers, digits, and the following special characters '@#$%^&-+=()!? '"
        )
    )]
    pub password: String,
    #[validate(must_match(
        other = "password",
        message = "confirm password should match password"
    ))]
    pub confirm_password: String,
}

#[derive(Debug, Deserialize, Insertable, Validate)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    password: String,
    created_at: NaiveDateTime,
    last_login: NaiveDateTime,
}

impl NewUser {
    pub fn from_user_form(user_form: UserForm) -> Result<Self, RippleError> {
        if user_form.password != user_form.confirm_password {
            return Err(RippleError::UserError(
                "confirm password does not match".to_string(),
            ));
        }

        dotenv().ok();

        let secret = std::env::var("SECRET_KEY").expect("SECRET_KEY must be set");

        let hash = Hasher::default()
            .with_password(user_form.password)
            .with_secret_key(secret)
            .hash()
            .unwrap();

        Ok(NewUser {
            username: user_form.username,
            email: user_form.email,
            password: hash,
            created_at: Local::now().naive_utc(),
            last_login: Local::now().naive_utc(),
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Insertable, AsChangeset)]
#[table_name = "spotify_tokens"]
pub struct NewSpotifyToken {
    username: String,
    access_token: String,
    expires_in_seconds: i64,
    expires_at: Option<NaiveDateTime>,
    refresh_token: Option<String>,
    scopes: Vec<String>,
}

impl NewSpotifyToken {
    pub fn from_token(username: String, token: Token) -> Self {
        NewSpotifyToken {
            username,
            access_token: token.access_token,
            expires_in_seconds: token.expires_in.num_seconds(),
            expires_at: match token.expires_at {
                Some(expires_at) => Some(expires_at.naive_utc()),
                None => None,
            },
            refresh_token: token.refresh_token,
            scopes: Vec::from_iter(token.scopes.iter().map(|s| s.clone())),
        }
    }
}

#[derive(Debug, Identifiable, Queryable, Serialize)]
pub struct SpotifyToken {
    id: i32,
    username: String,
    access_token: String,
    expires_in_seconds: i64,
    expires_at: Option<NaiveDateTime>,
    refresh_token: Option<String>,
    scopes: Vec<String>,
}

impl From<SpotifyToken> for Token {
    fn from(spotify_token: SpotifyToken) -> Self {
        Token {
            access_token: spotify_token.access_token.clone(),
            expires_in: Duration::seconds(spotify_token.expires_in_seconds),
            expires_at: match spotify_token.expires_at {
                Some(datetime) => Some(DateTime::<Utc>::from_utc(datetime, Utc)),
                None => None,
            },
            refresh_token: spotify_token.refresh_token.clone(),
            scopes: HashSet::from_iter(spotify_token.scopes.iter().map(|s| s.clone())),
        }
    }
}
