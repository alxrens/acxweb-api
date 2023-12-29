#[macro_use]
extern crate log;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};

use sea_orm::DatabaseConnection;

use std::env;

pub mod config;
mod errors;
pub mod db;
pub mod routes;

use db::{database_connection, migration};

#[derive(Debug, Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let base_url = env::var("BASE_URL").expect("BASE_URL is not set in .env file");
    let server_url = format!("{}:{}", host, port);

    let conn = database_connection().await.unwrap();
    migration(&conn).await.expect("Failed to run migrations");
    let state = AppState { conn };

    env_logger::init();

    let app = move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .wrap(Logger::default())
            .configure(config::init)
            .configure(routes::init)
    };

    info!("STARTING SEERVER AT : {}", base_url);
    HttpServer::new(app).bind(server_url)?.run().await
}