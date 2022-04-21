use actix_web::{web, get, post, delete, HttpResponse};
use crate::db::{Pool, user::{get_user_all, get_user_by_id, create_user, update_user, delete_user}};
use crate::models::user::{UserRequest, UserDB};
use crate::models::error::HttpErrors;


// READ
#[get("/get")]
pub async fn user_get_all(db: web::Data<Pool>) -> Result<HttpResponse, HttpErrors> {
    // request
    // ...

    // process
    let users = web::block(move || get_user_all(db)).await.unwrap(); if users.is_err() {
        return Err(HttpErrors::StandardError {
            message: "Failed to get users!".to_owned(),
        })
    }

    // response
    Ok(HttpResponse::Ok().json(users.unwrap()))
}

#[get("/get/{user_id}")]
pub async fn user_get(db: web::Data<Pool>, user_id: web::Path<i32>) -> Result<HttpResponse, HttpErrors> {
    // request
    let id = user_id.into_inner();

    // process
    let user = web::block(move || get_user_by_id(db, id)).await.unwrap(); if user.is_err() {
        return Err(HttpErrors::StandardError {
            message: "Failed to get user!".to_owned(),
        })
    }

    // response
    Ok(HttpResponse::Ok().json(user.unwrap()))
}

// WRITE
#[post("/create")]
pub async fn user_create(db: web::Data<Pool>, user: web::Json<UserRequest>) -> Result<HttpResponse, HttpErrors> {
    // request
    let mut user = user.into_inner();
    user.hash_password();

    // process
    let user = web::block(move || create_user(db, UserDB::generate(&user))).await.unwrap(); if user.is_err() {
        return Err(HttpErrors::StandardError {
            message: "Failed to create user!".to_owned(),
        })
    }

    // response
    Ok(HttpResponse::Ok().json(user.unwrap()))
}

#[post("/update/{user_id}")]
pub async fn user_update(db: web::Data<Pool>, user_id: web::Path<i32>, user: web::Json<UserRequest>) -> Result<HttpResponse, HttpErrors> {
    // request
    let id = user_id.into_inner();
    let mut user = user.into_inner();
    user.hash_password();

    // process
    let user = web::block(move || update_user(db, UserDB::generate(&user), id)).await.unwrap(); if user.is_err() {
        return Err(HttpErrors::StandardError {
            message: "Failed to update user!".to_owned(),
        })
    };

    // response
    Ok(HttpResponse::Ok().json(user.unwrap()))
}

#[delete("/delete/{user_id}")]
pub async fn user_delete(db: web::Data<Pool>, user_id: web::Path<i32>) -> Result<HttpResponse, HttpErrors> {
    // request
    let id = user_id.into_inner();

    // process
    let id = web::block(move || delete_user(db, id)).await.unwrap(); if id.is_err() {
        return Err(HttpErrors::StandardError {
            message: "Failed to delete user!".to_owned(),
        })
    };

    Ok(HttpResponse::Ok().body("Succeed to Delete User!"))
}