use actix_web::{web, get, HttpResponse, Error};
use crate::db::{Pool, user::get_all_users};


// READ
#[get("/get/{user_id}")]
pub async fn user_get(db: web::Data<Pool>, user_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    // request
    let id = user_id.into_inner();

    // process
    let users = web::block(move || get_all_users(db))
        .await
        .unwrap()
        .unwrap();

    // response
    Ok(HttpResponse::Ok().json(users))
}