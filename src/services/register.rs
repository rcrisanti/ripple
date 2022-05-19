use crate::diesel;
use crate::errors::RippleError;
use crate::models::{NewUser, UserForm};
use crate::schema;
use crate::Pool;
use actix_identity::Identity;
use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use tera::{Context, Tera};

pub async fn register(tera: web::Data<Tera>) -> Result<HttpResponse, RippleError> {
    let mut data = Context::new();
    data.insert("title", "register");

    let rendered = tera.render("register.html", &data)?;
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn process_registration(
    data: web::Form<UserForm>,
    id: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, RippleError> {
    use schema::users;

    let new_user = NewUser::from_user_form(data.into_inner())?;
    let connection = pool.get()?;

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&connection)?;

    log::info!("process registration for {}", new_user.email);

    id.remember(new_user.email.to_string());
    Ok(HttpResponse::Ok().body("processed registration"))
}
