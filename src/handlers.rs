// src/handlers.rs
use crate::models::Drug;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize)]
pub struct HealthCheckResponse {
    status: String,
}

pub async fn health_check() -> Json<HealthCheckResponse> {
    Json(HealthCheckResponse {
        status: "OK".to_string(),
    })
}

#[derive(Deserialize)]
pub struct SearchQuery {
    q: String,
}

pub async fn search_drugs(
    State(pool): State<PgPool>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<Vec<Drug>>, StatusCode> {
    let search_term = format!("%{}%", query.q);

    let drugs = sqlx::query_as!(
        Drug,
        r#"
        SELECT
            id, category_id, generic_name, syn_name, detail, drug_type,
            dosage_forms, ed_level, recommendations, conditions, warnings,
            notes, footnote, source_code
        FROM drugs
        WHERE generic_name ILIKE $1 OR syn_name ILIKE $1
        ORDER BY generic_name
        LIMIT 100
        "#,
        search_term
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to query drugs: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(drugs))
}

pub async fn get_drug_by_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Drug>, StatusCode> {
    sqlx::query_as!(
        Drug,
        r#"
        SELECT
            id, category_id, generic_name, syn_name, detail, drug_type,
            dosage_forms, ed_level, recommendations, conditions, warnings,
            notes, footnote, source_code
        FROM drugs
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch drug by id: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?
    .map(Json)
    .ok_or(StatusCode::NOT_FOUND)
}