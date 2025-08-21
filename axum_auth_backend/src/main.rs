mod models
mod config;
mod dtos;
mod error;
mod db;
mod utils;
mod middleware;

use axum::{
    {http::{header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE}, HeaderValue, Method}, Extension, Router, response::IntoResponse, routing::{get, post, put, delete, patch}};
}
use config::Config;
use db::DBClient;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::{CorsLayer, AllowOrigin};
use tracing::level_filters::LevelFilter;

#[derive(Clone, Debug)]
pub struct AppState {
   pub env: Config,
   pub db_client: DBClient,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init()
    .with_max_level(levelFilter::DEBUG)
    .init();

    dotenv().ok();

    let config = Config::init();

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await {
            Ok(pool) => {
                println!("Database connection pool created successfully.");
                pool
            },

            Err(e) => {
                eprintln!("Failed to connect to the database: {:?}", e);
                std::process::exit(1);
            }
        };

        let cors = CorsLayer::new()
            .allow_origin("http://localhost:8000".parse::<HeaderValue>().unwrap())
            .allow_headers([AUTHORIZATION, CONTENT_TYPE, ACCEPT])
            .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::PATCH])

        let db_client = DBClient::new(pool.clone());
    let app_state = AppState {
        env: config.clone(),
        db_client,
    };
    let app = Router::new()
        .layer(Extension(app_state));
        .layer(cors)

    println!("Server is running on http://localhost:{}", config.port); 
    
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", &config.port))
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app).await.unwrap();    
}
