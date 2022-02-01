use actix_web::{HttpResponse, Responder};

pub async fn root() -> impl Responder {
    HttpResponse::Ok().body("Root of API")
}
