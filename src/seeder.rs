// src/seeder.rs

//! The seeder module is responsible for parsing the NLEM CSV file
//! and populating the database. This is intended to be a one-time operation.

use sqlx::PgPool;
use std::error::Error;
use tracing::info;

/// Represents a single row from the source CSV file.
/// Only fields that are actually used for seeding are defined here
/// to avoid "unused field" warnings and keep the code clean.
#[derive(Debug, serde::Deserialize)]
struct CsvRecord {
    // --- Category Information ---
    grcode1: String, name1: String,
    grcode2: String, name2: String,
    grcode3: String, name3: String,
    grcode4: String, name4: String,
    
    // --- Drug Information ---
    #[serde(rename = "generic name")]
    generic_name: String,
    #[serde(rename = "syn name")]
    syn_name: String,
    #[serde(rename = "detail of generic name")]
    detail_of_generic_name: String,
    #[serde(rename = "ประเภทยา")]
    drug_type: String,
    dosage: String,
    #[serde(rename = "ED")]
    ed_level: String,
    #[serde(rename = "คำแนะนำ")]
    recommendations: String,
    #[serde(rename = "เงื่อนไข")]
    conditions: String,
    #[serde(rename = "คำเตือนและข้อควรระวัง")]
    warnings: String,
    #[serde(rename = "หมายเหตุ")]
    notes: String,
    #[serde(rename = "Footnote")]
    footnote: String,
    #[serde(rename = "Code ฉ.67")]
    source_code: String,
}

/// A helper function to trim whitespace and newline characters from a string slice.
/// Returns `None` if the resulting string is empty, otherwise returns `Some(String)`.
fn clean_string(s: &str) -> Option<String> {
    let cleaned = s.trim().replace('\n', " ");
    if cleaned.is_empty() {
        None
    } else {
        Some(cleaned)
    }
}

/// The main seeding function. It reads data from the specified CSV file,
/// processes it, and inserts it into the `drug_categories` and `drugs` tables.
/// This function is transactional; it will either complete fully or not at all.
pub async fn seed_data(pool: &PgPool) -> Result<(), Box<dyn Error>> {
    info!("Starting database seeding process...");

    // Use a transaction to ensure data integrity.
    // If any part of the process fails, all changes will be rolled back.
    let mut tx = pool.begin().await?;

    info!("Clearing existing data to prevent duplicates...");
    sqlx::query("DELETE FROM drugs").execute(&mut *tx).await?;
    sqlx::query("DELETE FROM drug_categories").execute(&mut *tx).await?;
    
    let mut rdr = csv::Reader::from_path("./data/nlem_2567.csv")?;
    
    // State variables to keep track of the current category hierarchy
    // as we iterate through the CSV rows.
    let mut current_cat1_id: Option<i32> = None;
    let mut current_cat2_id: Option<i32> = None;
    let mut current_cat3_id: Option<i32> = None;
    let mut current_cat4_id: Option<i32> = None;

    info!("Reading CSV and inserting data...");
    for result in rdr.deserialize() {
        let record: CsvRecord = result?;
        
        // --- Category Processing ---
        // Process categories from level 1 down to 4.
        // `ON CONFLICT DO UPDATE` ensures that if a category code already exists,
        // we just update its name, preventing duplicate entries.
        if let Some(name1) = clean_string(&record.name1) {
            let code = record.grcode1.trim();
            let row = sqlx::query!("INSERT INTO drug_categories (code, name, level) VALUES ($1, $2, 1) ON CONFLICT (code) DO UPDATE SET name = EXCLUDED.name RETURNING id", code, name1)
                .fetch_one(&mut *tx).await?;
            current_cat1_id = Some(row.id);
        }
        
        if let Some(name2) = clean_string(&record.name2) {
            let code = format!("{}.{}", record.grcode1.trim(), record.grcode2.trim());
            let row = sqlx::query!("INSERT INTO drug_categories (code, name, level, parent_id) VALUES ($1, $2, 2, $3) ON CONFLICT (code) DO UPDATE SET name = EXCLUDED.name RETURNING id", code, name2, current_cat1_id)
                .fetch_one(&mut *tx).await?;
            current_cat2_id = Some(row.id);
        }

        if let Some(name3) = clean_string(&record.name3) {
            let code = format!("{}.{}.{}", record.grcode1.trim(), record.grcode2.trim(), record.grcode3.trim());
             let row = sqlx::query!("INSERT INTO drug_categories (code, name, level, parent_id) VALUES ($1, $2, 3, $3) ON CONFLICT (code) DO UPDATE SET name = EXCLUDED.name RETURNING id", code, name3, current_cat2_id)
                .fetch_one(&mut *tx).await?;
            current_cat3_id = Some(row.id);
        }

        if let Some(name4) = clean_string(&record.name4) {
            let code = format!("{}.{}.{}.{}", record.grcode1.trim(), record.grcode2.trim(), record.grcode3.trim(), record.grcode4.trim());
             let row = sqlx::query!("INSERT INTO drug_categories (code, name, level, parent_id) VALUES ($1, $2, 4, $3) ON CONFLICT (code) DO UPDATE SET name = EXCLUDED.name RETURNING id", code, name4, current_cat3_id)
                .fetch_one(&mut *tx).await?;
            current_cat4_id = Some(row.id);
        }

        // --- Drug Processing ---
        // A row is considered a drug entry if it has a non-empty generic name.
        if let Some(generic_name) = clean_string(&record.generic_name) {
            // Determine the drug's category by finding the most specific (deepest) one available.
            let category_id = current_cat4_id.or(current_cat3_id).or(current_cat2_id).or(current_cat1_id);
            
            // The dosage form is a comma-separated string; we parse it into a Vec<String>.
            let dosage_forms: Vec<String> = record.dosage.split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            
            sqlx::query!(
                r#"
                INSERT INTO drugs (
                    category_id, generic_name, syn_name, detail, drug_type, dosage_forms,
                    ed_level, recommendations, conditions, warnings, notes, footnote, source_code
                ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
                "#,
                category_id,
                generic_name,
                clean_string(&record.syn_name),
                clean_string(&record.detail_of_generic_name),
                clean_string(&record.drug_type),
                &dosage_forms, // Note: Pass Vec<String> as a slice for sqlx
                clean_string(&record.ed_level),
                clean_string(&record.recommendations),
                clean_string(&record.conditions),
                clean_string(&record.warnings),
                clean_string(&record.notes),
                clean_string(&record.footnote),
                clean_string(&record.source_code)
            )
            .execute(&mut *tx).await?;
        }
    }
    
    // Commit the transaction to save all changes to the database.
    tx.commit().await?;
    info!("✅ Database seeding completed successfully!");
    Ok(())
}