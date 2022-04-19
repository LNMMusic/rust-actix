use actix_web::{web, Scope};
// scopes - group routes
pub mod root; pub mod api; pub mod user;


// ROUTER
pub fn scope_router() -> Scope {
    web::scope("")
        // root
        .route("/", web::get().to(root::root))

        // api
        .service(web::scope("/api")
            .service(api::api_get)
            .service(api::api_post)
            .service(api::api_post_param)
        )

        // user
        .service(web::scope("/user")
            .service(user::user_get)
        )
}