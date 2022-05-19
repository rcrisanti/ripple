use actix_identity::Identity;
use actix_web::{HttpResponse, Responder};

pub async fn process_logout(id: Identity) -> impl Responder {
    id.forget();
    HttpResponse::Ok().body("logged out")
}
