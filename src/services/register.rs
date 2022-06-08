use crate::diesel;
use crate::errors::RippleError;
use crate::models::{NewUser, UserForm};
use crate::schema;
use crate::Pool;
use actix_identity::Identity;
use actix_web::{get, post, web, HttpResponse};
use diesel::prelude::*;
use tera::{Context, Tera};
use validator::{Validate, ValidationErrors, ValidationErrorsKind};

#[get("/register")]
pub async fn register(tera: web::Data<Tera>, id: Identity) -> Result<HttpResponse, RippleError> {
    let mut data = Context::new();
    data.insert("title", "register");

    if let Some(my_username) = id.identity() {
        log::debug!("logged in");
        data.insert("logged_in", "true");
        data.insert("my_username", &my_username);
    } else {
        log::debug!("not logged in");
        data.insert("logged_in", "false");
    }

    let rendered = tera.render("register.html", &data)?;
    Ok(HttpResponse::Ok().body(rendered))
}

async fn register_with_warnings(
    tera: web::Data<Tera>,
    id: Identity,
    e: ValidationErrors,
) -> Result<HttpResponse, RippleError> {
    let mut data = Context::new();
    data.insert("title", "register");
    if let Some(my_username) = id.identity() {
        log::debug!("logged in");

        data.insert("logged_in", "true");
        data.insert("my_username", &my_username);
    } else {
        log::debug!("not logged in");

        data.insert("logged_in", "false");
    }

    for (field, errors) in e.errors().into_iter() {
        if let ValidationErrorsKind::Field(errors) = errors {
            let err = errors.first().unwrap();
            data.insert(format!("{}_error", field), &err.message);
        }
    }

    let rendered = tera.render("register.html", &data)?;
    Ok(HttpResponse::Ok().body(rendered))
}

#[post("/register")]
pub async fn process_registration(
    tera: web::Data<Tera>,
    web::Form(user_form): web::Form<UserForm>,
    id: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, RippleError> {
    log::debug!("posting registration user form");

    if let Err(e) = user_form.validate() {
        // return Ok(HttpResponse::Ok().body(format!("Registration error: {}", e)));
        // return Ok(HttpResponse::SeeOther()
        //     .set_header("Location", "/register")
        //     .finish());

        return register_with_warnings(tera, id, e).await;
    }

    use schema::users;

    let new_user = NewUser::from_user_form(user_form)?;
    let connection = pool.get()?;

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&connection)?;

    log::info!("process registration for {}", new_user.username);

    id.remember(new_user.username.to_string());
    log::debug!("remember session id");
    Ok(HttpResponse::Ok().body("processed registration"))
}
