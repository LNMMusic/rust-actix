use actix_web::{App, HttpServer, web};
use api::routes;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(routes::manual_hello))
            // apps
            // .service(routes::hello)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}