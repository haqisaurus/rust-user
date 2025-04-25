use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use std::env;
use std::time::Duration;

pub async fn setup_database() -> DatabaseConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let is_sql_log = env::var("ENABLE_SQL_LOG").expect("SQL log must be set");

    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(100)
    .min_connections(5)
    .connect_timeout(Duration::from_secs(8))
    .acquire_timeout(Duration::from_secs(8))
    .idle_timeout(Duration::from_secs(8))
    .max_lifetime(Duration::from_secs(8))
    .sqlx_logging(is_sql_log.eq("true"))
    .sqlx_logging_level(log::LevelFilter::Info)

    .set_schema_search_path("public"); // Setting default PostgreSQL schema

    let db : Result<DatabaseConnection, DbErr> = Database::connect(opt).await;
    if let Err(e) = db {
        panic!("Database connection error: {}", e);
    }
    db.unwrap()
}
