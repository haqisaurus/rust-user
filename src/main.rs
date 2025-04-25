mod config;
mod dto;
mod handlers;
mod models;
mod routes;
mod services;
mod utils;

use actix_web::body::BoxBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::middleware::{Logger, Next, from_fn};
use actix_web::{App, Error, HttpMessage, HttpResponse, HttpServer, web};
use dotenv::dotenv;
use dto::common_dto::Claims;
use env_logger::Env;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use routes::user_routes::init_routes;
use sea_orm::DatabaseConnection;
use std::env;
use crate::dto::error_dto::ErrorResponse;

#[derive(Debug, Clone)]
struct AppState {
    conn: DatabaseConnection,
}

async fn member_middleware(
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<BoxBody>, Error> {
    let token = req
        .headers()
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|auth| auth.strip_prefix("Bearer "))
        .unwrap_or("");

    if req.path() == "/api/login" || req.path() == "/api/logout" || req.path() == "/api/register" {
        return next.call(req).await.map(|res| res.map_into_boxed_body());
    }

    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_audience(&["me"]);
    validation.set_required_spec_claims(&["exp", "sub", "iss", "company"]);

    let private_key = env::var("JWT_SECRET").unwrap_or_else(|_| "walla".into());

    match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(private_key.as_ref()),
        &validation,
    ) {
        Ok(token_data) => {
            println!("{:?}", token_data.claims);
            req.extensions_mut().insert(token_data.claims);
            next.call(req).await.map(|res| res.map_into_boxed_body())
        }
        Err(err) => {
            let message = match *err.kind() {
                ErrorKind::InvalidToken => "Token is invalid",
                ErrorKind::InvalidIssuer => "Issuer is invalid",
                ErrorKind::ExpiredSignature => "Token has expired",
                _ => "Authorization failed",
            };

            let response = HttpResponse::Unauthorized()
                .content_type("application/json")
                .json(ErrorResponse {
                    code: 401004,
                    message: message.to_string(),
                    data: "".to_string(),
                });

            Ok(req.into_response(response.map_into_boxed_body()))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let debug_level = env::var("DEBUG_LEVEL").unwrap_or_else(|_| "info".to_string());
    env_logger::init_from_env(Env::default().default_filter_or(debug_level));
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    println!("Server running at https://{}:{}", host, port);
    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::DEBUG)
    //     .with_test_writer()
    //     .init();

    let db_pool = config::db::setup_database().await;
    let state = AppState {
        conn: db_pool,
    };

    HttpServer::new(move || {
        App::new()
            .wrap(from_fn(member_middleware))
            .wrap(Logger::default())
            .app_data(web::Data::new(state.clone()))
            .configure(init_routes)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
