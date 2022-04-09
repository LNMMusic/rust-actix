// LIBS
use actix_web::{web, get, post, HttpResponse, Responder, Error};
use deadpool_postgres::{Client, Pool};
use crate::{models::user::User, config::error::MyError, db};


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

#[post("/create")]
pub async fn user_create(req: web::Json<User>, db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    // request
    let mut req = req.into_inner();

    let client: Client = db_pool
        .get()
        .await
        .map_err(MyError::PoolError)?;
    

    // process
    req.hash_password();
    let user = db::user::create_user(&client, req).await?;
    
    // response
    Ok(HttpResponse::Ok().json(user))
}