// LIBS
use actix_web::{web, App, HttpServer};
use crate::{db::Pool, routes};


// APP Server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Config
    dotenv::dotenv().ok();

    // DB
    let db_uri = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let client = ConnectionManager::<PgConnection>::new(db_uri);
    let pool: Pool = r2d2::Pool::builder()
                .build(client)
                .expect("Failed to create pool.");

    // Server
    let server = HttpServer::new(move || {
        // Router
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(routes::scope_root())
            .service(routes::scope_api())
            .service(routes::scope_user())
    })
    .workers(4)
    .bind(("0.0.0.0", 8080))?
    .run();
    server.await
}