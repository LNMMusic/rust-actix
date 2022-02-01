use actix_web::{web, Scope};
// scopes - group routes
pub mod root;
pub mod api;


pub fn scope_root() -> Scope {
    let scope = web::scope("/")
                    .route("", web::get().to(root::root));
    scope
}

pub fn scope_api() -> Scope {
    let scope = web::scope("/api")
                    .service(api::echo)
                    .service(api::hello);
    scope
}