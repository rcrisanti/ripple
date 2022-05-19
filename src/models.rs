use super::errors::RippleError;
use crate::schema::users;

use argonautica::Hasher;
use diesel::{Identifiable, Insertable, Queryable};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};

#[derive(Debug, Identifiable, Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: chrono::NaiveDateTime,
    pub last_login: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct UserForm {
    pub username: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

#[derive(Debug, Deserialize, Insertable)]
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
