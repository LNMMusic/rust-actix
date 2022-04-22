use actix_web::{get, post, HttpResponse};
use crate::models::error::HttpErrors;


#[post("/sign-in")]
pub async fn auth_sign_in() -> Result<HttpResponse, HttpErrors> {
    Ok(HttpResponse::Ok().body("signed in"))
}

#[get("/sign-out")]
pub async fn auth_sign_out() -> Result<HttpResponse, HttpErrors> {
    Ok(HttpResponse::Ok().body("signed out"))
}