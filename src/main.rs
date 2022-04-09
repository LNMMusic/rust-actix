// LIBS
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use tokio_postgres::NoTls;
use api::{routes, config};


// APP Server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Config
    dotenv().ok();

    // DB
    let config = config::ConfigApp::from_env().unwrap();
    let pool = config.pg.create_pool(None, NoTls).unwrap();

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