use super::errors::RippleError;
use crate::schema::users;

use argonautica::Hasher;
use diesel::{Identifiable, Insertable, Queryable};
use dotenv::dotenv;
use regex::Regex;
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
    pub created_at: chrono::NaiveDateTime,
    pub last_login: chrono::NaiveDateTime,
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
    created_at: chrono::NaiveDateTime,
    last_login: chrono::NaiveDateTime,
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
            created_at: chrono::Local::now().naive_utc(),
            last_login: chrono::Local::now().naive_utc(),
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}
