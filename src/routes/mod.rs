use actix_web::{web, Scope};
// scopes - group routes
mod root; mod api; mod user; mod auth;


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
            // read
            .service(user::user_get)
            .service(user::user_get_all)
            // write
            .service(user::user_create)
        )

        //auth
        .service(web::scope("/auth")
            .service(auth::auth_sign_in)
            .service(auth::auth_sign_out)
        )
}