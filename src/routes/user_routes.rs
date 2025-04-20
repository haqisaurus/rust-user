use actix_web::web;
use crate::handlers::auth_handler::{post_login, post_register};
use crate::handlers::user_handler::{get_test_join_json, get_test_nested_json, get_users};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/login", web::post().to(post_login))
            .route("/users", web::get().to(get_users))
            .route("/test-nested-json", web::get().to(get_test_nested_json))
            .route("/test-join-json", web::get().to(get_test_join_json))
            .route("/register", web::post().to(post_register))
    );
}