use actix_web::{web, get, post, delete, HttpResponse, Error};
use crate::db::{Pool, user::{get_user_all, get_user_by_id, create_user, update_user, delete_user}};
use crate::models::user::{UserRequest, UserDB};


// READ
#[get("/get")]
pub async fn user_get_all(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    // request
    // ...

    // process
    let users = web::block(move || get_user_all(db))
        .await
        .unwrap()
        .unwrap();

    // response
    Ok(HttpResponse::Ok().json(users))
}

#[get("/get/{user_id}")]
pub async fn user_get(db: web::Data<Pool>, user_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    // request
    let id = user_id.into_inner();

    // process
    let user = web::block(move || get_user_by_id(db, id))
        .await
        .unwrap()
        .unwrap();

    // response
    Ok(HttpResponse::Ok().json(user))
}

// WRITE
#[post("/create")]
pub async fn user_create(db: web::Data<Pool>, user: web::Json<UserRequest>) -> Result<HttpResponse, Error> {
    // request
    let mut user = user.into_inner();
    user.hash_password();

    // process
    let user = web::block(move || create_user(db, UserDB::generate(&user)))
        .await
        .unwrap()
        .unwrap();

    // response
    Ok(HttpResponse::Ok().json(user))
}

#[post("/update/{user_id}")]
pub async fn user_update(db: web::Data<Pool>, user_id: web::Path<i32>, user: web::Json<UserRequest>) -> Result<HttpResponse, Error> {
    // request
    let id = user_id.into_inner();
    let mut user = user.into_inner();
    user.hash_password();

    // process
    let user = web::block(move || update_user(db, UserDB::generate(&user), id))
        .await
        .unwrap()
        .unwrap();

    // response
    Ok(HttpResponse::Ok().json(user))
}

#[delete("/delete/{user_id}")]
pub async fn user_delete(db: web::Data<Pool>, user_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    // request
    let id = user_id.into_inner();

    // process
    let _id = web::block(move || delete_user(db, id))
        .await
        .unwrap()
        .unwrap();
    
    Ok(HttpResponse::Ok().body("Succeed to Delete User!"))
}