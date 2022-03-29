// LIBS
use actix_web::{App, HttpServer};
use api::routes;


// APP Server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
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