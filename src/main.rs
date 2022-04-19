// Modules
#[macro_use]
extern crate diesel;
mod config; mod models; mod db; mod routes;

// Libs
use actix_web::{web, App, HttpServer};
use diesel::{prelude::*, r2d2::{self, ConnectionManager}};


// APP Server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // CONFIG
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");

    // DB
    let db_uri = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let client = ConnectionManager::<PgConnection>::new(db_uri);
    let pool: db::Pool = r2d2::Pool::builder()
                .build(client)
                .expect("Failed to create pool.");

    // SERVER
    HttpServer::new(move || {
        App::new()
            // db
            .app_data(web::Data::new(pool.clone()))
            // Router
            .service(routes::scope_router())
    })
    .workers(4)
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}