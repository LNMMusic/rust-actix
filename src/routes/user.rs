// LIBS
use actix_web::{get, HttpResponse, Responder};
use crate::models::user::User;


#[get("/get")]
pub async fn user_get() -> impl Responder {
    // request
    // ...


    // process
    let mut users = vec![
        User {
            username: String::from("lnm"),
            password: String::from("password"),
            ..Default::default()
        },
        User {
            username: String::from("test"),
            password: String::from("password"),
            ..Default::default()
        },
    ];

    for user in &mut users {
        user.hash_password();
    };

    // response
    HttpResponse::Ok().json(users)
}