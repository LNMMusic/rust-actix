use actix_web::{HttpResponse, Responder};
// use actix_web::{get, post};


// root [normal route without method - assigned in main.rs] -> route attr
pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Root of API")
}

// others [normal route with method - ]                     -> service attr
// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }

// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

// routes
pub mod users;