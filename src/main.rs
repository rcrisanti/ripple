mod errors;
mod models;
pub mod schema;
mod services;

#[macro_use]
extern crate diesel;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use tera::Tera;

use services::{index, login, logout, register};

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
            .data(tera)
            .data(pool.clone())
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
            .service(web::resource("/logout").route(web::get().to(logout::process_logout)))
    })
    .bind("127.0.0.1:8001")?
    .run()
    .await
}
