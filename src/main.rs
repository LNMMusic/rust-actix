use actix_web::{App, HttpServer, };
use api::routes;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(routes::scope_root())
            .service(routes::scope_api())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}