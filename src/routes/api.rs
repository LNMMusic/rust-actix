// LIBS
use actix_web::{web, get, post, HttpResponse, Responder};
use serde::{Serialize, Deserialize};      use serde_json;


// READ
#[get("/")]
pub async fn api_get() -> impl Responder {
    HttpResponse::Ok().body("Root of API!")
}


// WRITE
#[post("/request")]
pub async fn api_post(req: String) -> impl Responder {
    // request
    // ...

    // process

    // response
    HttpResponse::Ok().body(req)
}

#[derive(Serialize, Deserialize)]
pub struct UserRequest {
    username:   String,
    code:       u32,
}
#[post("/request/{id}")]
pub async fn api_post_param(param: web::Path<u32>, req: web::Json<UserRequest>) -> impl Responder {
    // request
    let id = param.into_inner();

    // process
    println!("this is the id paramenter -> {}", id);
    println!("this is the request -> {} - {}", req.username, req.code);

    // response
    HttpResponse::Ok().body(serde_json::to_string(&req).unwrap())
}