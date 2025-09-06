-- migrations/{timestamp}_create_initial_tables.sql
CREATE TABLE drug_categories (
    id SERIAL PRIMARY KEY,
    code TEXT UNIQUE NOT NULL,
    name TEXT NOT NULL,
    level INTEGER NOT NULL,
    parent_id INTEGER REFERENCES drug_categories(id)
);

CREATE TABLE drugs (
    id SERIAL PRIMARY KEY,
    category_id INTEGER REFERENCES drug_categories(id),
    generic_name TEXT NOT NULL,
    syn_name TEXT,
    detail TEXT,
    drug_type TEXT,
    dosage_forms TEXT[] NOT NULL DEFAULT '{}',
    ed_level VARCHAR(10),
    recommendations TEXT,
    conditions TEXT,
    warnings TEXT,
    notes TEXT,
    footnote TEXT,
    source_code VARCHAR(20) -- Code ฉ.67
);-- Add migration script here
