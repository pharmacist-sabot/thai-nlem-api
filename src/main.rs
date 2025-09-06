// src/main.rs
mod db;
mod handlers;
mod models;
mod routes;
mod seeder;

use std::env;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup tracing/logging
    tracing_subscriber::fmt()
        .with_target(false)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Load .env file
    dotenvy::dotenv().ok();

    // Create database connection pool
    let pool = db::create_pool().await?;

    // Check for "seed" command-line argument
    let args: Vec<String> = env::args().collect();
    if args.get(1) == Some(&"seed".to_string()) {
        seeder::seed_data(&pool).await?;
        return Ok(()); // Exit after seeding
    }

    // Create the Axum router
    let app = routes::create_router(pool);

    // Run the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    info!("🚀 Server listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
