use actix_identity::Identity;
use actix_web::{get, HttpResponse, Responder};

#[get("/logout")]
pub async fn process_logout(id: Identity) -> impl Responder {
    id.forget();
    log::debug!("forgot session token");
    HttpResponse::Ok().body("logged out")
}
