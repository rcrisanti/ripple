mod errors;
mod models;
mod schema;
mod services;
mod spotify;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use tera::Tera;

use services::{account, home, index, login, logout, register, spotify_connect, user_profile};

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

// fn print_random_string() {
//     // for generating a hash key
//     use rand::{distributions::Alphanumeric, Rng};
//     let s: String = rand::thread_rng()
//         .sample_iter(&Alphanumeric)
//         .take(100)
//         .map(char::from)
//         .collect();
//     println!("{}", s);
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    let database_url = std::env::var("DATABASE_URL").unwrap();

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create Postgres pool.");

    HttpServer::new(move || {
        let tera = Tera::new("templates/**/*").unwrap();

        App::new()
            .wrap(Logger::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])
                    .name("auth-cookie")
                    .secure(false),
            ))
            .app_data(web::Data::new(tera))
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(index::index))
            .service(
                web::resource("/login")
                    .route(web::get().to(login::login))
                    .route(web::post().to(login::process_login)),
            )
            .service(
                web::resource("/register")
                    .route(web::get().to(register::register))
                    .route(web::post().to(register::process_registration)),
            )
            .route("/logout", web::get().to(logout::process_logout))
            .service(
                web::resource("/user/{username}").route(web::get().to(user_profile::user_profile)),
            )
            .route("/home", web::get().to(home::home))
            .route("/account", web::get().to(account::account))
            .route(
                "/spotify-connect",
                web::get().to(spotify_connect::spotify_connect),
            )
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
