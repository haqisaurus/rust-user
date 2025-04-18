use actix_web::web;
use crate::handlers::auth_handler::login;
use crate::handlers::user_handler::{get_users};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/login", web::post().to(login))
            .route("/users", web::get().to(get_users))
    );
}