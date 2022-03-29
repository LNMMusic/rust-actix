use actix_web::{web, Scope};
// scopes - group routes
pub mod root;
pub mod api;


// ROUTER
pub fn scope_root() -> Scope {
    let scope = web::scope("/")
                    // services
                    .route("", web::get().to(root::root));
    scope
}

pub fn scope_api() -> Scope {
    let scope = web::scope("/api")
                    // services
                    .service(api::api_get)
                    .service(api::api_post)
                    .service(api::api_post_param);
    scope
}