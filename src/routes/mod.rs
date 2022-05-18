use actix_web::{web, Scope};
use actix_web_httpauth::middleware::HttpAuthentication;
use crate::handlers::jwt::middleware2::jwt_validation;

// scopes - group routes
mod root; mod api; mod user; mod auth;


// ROUTER
pub fn scope_router() -> Scope {
    let jwt_middleware = HttpAuthentication::bearer(jwt_validation);

    web::scope("")
        // root
        .route("/", web::get().to(root::root))

        // api
        .service(web::scope("/api")
            .wrap(jwt_middleware)
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
            .service(auth::auth_sign_up)
            .service(auth::auth_sign_in)
            .service(auth::auth_sign_out)
        )
}