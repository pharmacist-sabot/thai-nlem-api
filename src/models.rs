// src/models.rs
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize)]
#[allow(dead_code)]
pub struct DrugCategory {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub level: i32,
    pub parent_id: Option<i32>,
}

#[derive(Debug, FromRow, Serialize)]
pub struct Drug {
    pub id: i32,
    pub category_id: Option<i32>,
    pub generic_name: String,
    pub syn_name: Option<String>,
    pub detail: Option<String>,
    pub drug_type: Option<String>,
    pub dosage_forms: Vec<String>,
    pub ed_level: Option<String>,
    pub recommendations: Option<String>,
    pub conditions: Option<String>,
    pub warnings: Option<String>,
    pub notes: Option<String>,
    pub footnote: Option<String>,
    pub source_code: Option<String>,
}