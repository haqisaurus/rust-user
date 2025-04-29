use crate::handlers::auth_handler::{activation_user, company_list, post_login, post_register, refresh_token};
use crate::handlers::permission_handler::{permission_delete, permission_detail, permission_list, permission_save};
use crate::handlers::role_handler::{role_delete, role_detail, role_list, role_save};
use crate::handlers::user_handler::{error_handler, get_test_join_json, get_test_nested_json, get_users, user_save};
use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/login", web::post().to(post_login))
            .route("/refresh-token", web::post().to(refresh_token))
            .route("/users", web::get().to(get_users))
            .route("/test-nested-json", web::get().to(get_test_nested_json))
            .route("/test-join-json", web::get().to(get_test_join_json))
            .route("/test-error", web::get().to(error_handler))
            .route("/activation/{key}", web::get().to(activation_user))
            .route("/register", web::post().to(post_register))
            .route("/registered-company", web::get().to(company_list))

            .route("/user/save", web::post().to(user_save))

            .route("/permissions", web::get().to(permission_list))
            .route("/permission/{id}", web::get().to(permission_detail))
            .route("/permission/{id}", web::delete().to(permission_delete))
            .route("/permission/save", web::post().to(permission_save))

            .route("/roles", web::get().to(role_list))
            .route("/role/{id}", web::get().to(role_detail))
            .route("/role/{id}", web::delete().to(role_delete))
            .route("/role/save", web::post().to(role_save))

    );
}