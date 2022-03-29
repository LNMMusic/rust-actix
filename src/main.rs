// LIBS
use actix_web::{App, HttpServer};
use api::routes;
use dotenv::dotenv;
use std::env;


// APP Server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Env
    dotenv().ok();
    if let Ok(var) = env::var("DBPSQL_URI") {
        println!("var -> {}", var);
    };

    // Server
    HttpServer::new(|| {
        // Router
        App::new()
            .service(routes::scope_root())
            .service(routes::scope_api())
            .service(routes::scope_user())
    })
    .workers(4)
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}