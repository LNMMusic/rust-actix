use actix_web::{web, get, HttpResponse, Error};
use crate::db::{Pool, user::get_all_users};


// READ
#[get("/")]
pub async fn user_get(db: web::Data<Pool>, user_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || get_all_users(db))
        .await
        .map(|users| HttpResponse::Ok().json(users))
        .map_err(|_| HttpResponse::InternalServerError())?)
}