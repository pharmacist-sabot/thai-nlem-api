// src/routes.rs
use crate::handlers;
use axum::{
    routing::{get},
    Router,
};
use sqlx::PgPool;

pub fn create_router(pool: PgPool) -> Router {
    Router::new()
        .route("/", get(handlers::health_check))
        .route("/api/drugs/search", get(handlers::search_drugs))
        .route("/api/drugs/:id", get(handlers::get_drug_by_id))
        .with_state(pool)
}