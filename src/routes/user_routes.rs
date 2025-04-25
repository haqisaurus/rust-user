use crate::handlers::auth_handler::{post_login, post_register};
use crate::handlers::user_handler::{error_handler, get_test_join_json, get_test_nested_json, get_users};
use actix_web::web;
use crate::handlers::permission_handler::{permission_delete, permission_detail, permission_list, permission_save};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/login", web::post().to(post_login))
            .route("/users", web::get().to(get_users))
            .route("/test-nested-json", web::get().to(get_test_nested_json))
            .route("/test-join-json", web::get().to(get_test_join_json))
            .route("/test-error", web::get().to(error_handler))
            .route("/register", web::post().to(post_register))

            .route("/permissions", web::get().to(permission_list))
            .route("/permission/{id}", web::get().to(permission_detail))
            .route("/permission/{id}", web::delete().to(permission_delete))
            .route("/permission/save", web::post().to(permission_save))
    );
}