use actix_web::{web, get, post, HttpResponse};
use crate::db::{Pool, user::{get_user_by_username, create_user}};
use crate::models::error::HttpErrors;
use crate::models::user::{UserDB, UserSignIn, UserRequest, password};
use crate::handlers::jwt::{JWTCONFIG, Claims};


#[post("/sign-up")]
pub async fn auth_sign_up(db: web::Data<Pool>, user: web::Json<UserRequest>) -> Result<HttpResponse, HttpErrors> {
    // request
    let mut user = user.into_inner();
    password::hash(&mut user.password);

    // process
    let user = web::block(move || create_user(db, UserDB::generate(&user))).await.unwrap(); if user.is_err() {
        return Err(HttpErrors::StandardError {
            message: "Failed to Sign Up!".to_owned(),
        })
    }

    // response
    Ok(HttpResponse::Ok().json(user.unwrap()))
}

#[post("/sign-in")]
pub async fn auth_sign_in(db: web::Data<Pool>, user: web::Json<UserSignIn>) -> Result<HttpResponse, HttpErrors> {
    // request
    let user = user.into_inner();
    
    // process
    let user_db = web::block(move || get_user_by_username(db, &user.username)).await.unwrap(); if user_db.is_err() {
        return Err(HttpErrors::StandardError {
            message: "Failed to Sign In. Invalid Username!".to_owned(),
        })
    }; let user_db = user_db.unwrap();

    if !password::validate(&user.password, &user_db.password) {
        return Err(HttpErrors::StandardError {
            message: "Failed to Sign In. Invalid Password!".to_owned(),
        })
    };

    // token
    let token = JWTCONFIG.encoder.create_token(Claims::new(3, false)); if token.is_err() {
        return Err(HttpErrors::StandardError {
            message: "Failed to Sign In. Internal Error on Server to Create Token!".to_owned(),
        })
    } let token = token.unwrap();

    Ok(HttpResponse::Ok().body(token))
}

#[get("/sign-out")]
pub async fn auth_sign_out() -> Result<HttpResponse, HttpErrors> {
    Ok(HttpResponse::Ok().body("Succeed to Sign Out!"))
}