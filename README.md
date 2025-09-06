# Thai NLEM API

![Rust Version](https://img.shields.io/badge/rust-1.77%2B-orange.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Build Status](https://img.shields.io/github/actions/workflow/status/your-username/thai-nlem-api/rust.yml?branch=main) <!-- TODO: Replace with your actual username/repo -->

An open-source, unofficial REST API for accessing drug information from Thailand's National List of Essential Medicines (NLEM). Built with Rust, Axum, and SQLx for performance, safety, and reliability.

The goal of this project is to provide a clean, accessible, and well-documented API for developers, researchers, and healthcare professionals who need programmatic access to Thailand's essential medicine data.

## ✨ Features

-   **Fast & Efficient:** Built on the high-performance Axum web framework in Rust.
-   **Type-Safe:** Leverages SQLx for compile-time checked SQL queries.
-   **Search Functionality:** Powerful search endpoint for finding drugs by generic or synonym names.
-   **Structured Data:** Provides drug information, including dosage forms, ED level, warnings, and conditions in a clean JSON format.
-   **Open Source:** Free to use, modify, and contribute to under the MIT License.

## 🚀 Getting Started

Follow these instructions to get a local copy of the project up and running for development and testing purposes.

### Prerequisites

-   **Rust Toolchain:** Install from [rustup.rs](https://rustup.rs/).
-   **PostgreSQL:** A running PostgreSQL instance. We recommend using [Docker](https://www.docker.com/products/docker-desktop/).
-   **`sqlx-cli`:** For database migrations. Install with `cargo install sqlx-cli`.
-   **Source Data:** The project requires the `nlem_2567.csv` data file. (Please provide source/link if available).

### Installation & Setup

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/your-username/thai-nlem-api.git
    cd thai-nlem-api
    ```
    <!-- TODO: Replace with your actual username/repo -->

2.  **Setup the Database:**
    -   If using Docker, start a PostgreSQL container:
        ```bash
        docker run --name nlem-db -e POSTGRES_USER=myuser -e POSTGRES_PASSWORD=mypassword -e POSTGRES_DB=nlem_api_db -p 5432:5432 -d postgres:16-alpine
        ```
    -   Create a `.env` file from the example. Copy the content of `.env.example` (or create it manually).
        ```bash
        cp .env.example .env
        ```
    -   Update your `.env` file with your database connection string:
        ```env
        DATABASE_URL="postgres://myuser:mypassword@localhost:5432/nlem_api_db"
        ```

3.  **Run Database Migrations:**
    This command will create the necessary tables (`drugs`, `drug_categories`) in your database.
    ```bash
    sqlx database create # Only needed the first time
    sqlx migrate run
    ```

4.  **Seed the Database:**
    Place the `nlem_2567.csv` file inside the `data/` directory. Then, run the seeder to populate the database. This might take a few minutes.
    ```bash
    cargo run -- seed
    ```

5.  **Run the API Server:**
    Once seeding is complete, you can start the API server.
    ```bash
    cargo run
    ```
    The server will be running at `http://localhost:3000`.

## 📖 API Usage

The API provides several endpoints to access the drug data.

### Health Check

-   **Endpoint:** `GET /`
-   **Description:** Checks if the API server is running.
-   **Response:**
    ```json
    {
      "status": "OK"
    }
    ```

### Search for Drugs

-   **Endpoint:** `GET /api/drugs/search`
-   **Query Parameter:** `q` (string, required) - The search term.
-   **Description:** Searches for drugs where the generic name or synonym name matches the query (case-insensitive).
-   **Example:** `GET /api/drugs/search?q=paracetamol`
-   **Response:** An array of drug objects.
    ```json
    [
      {
        "id": 1,
        "category_id": 123,
        "generic_name": "Paracetamol",
        "syn_name": "Acetaminophen",
        "detail": null,
        "drug_type": null,
        "dosage_forms": ["tab", "syr"],
        "ed_level": "ก",
        "recommendations": "แนะนำให้ใช้เป็น first-line drug สำหรับ osteoarthritis",
        "conditions": null,
        "warnings": null,
        "notes": null,
        "footnote": null,
        "source_code": "S"
      }
    ]
    ```

### Get Drug by ID

-   **Endpoint:** `GET /api/drugs/:id`
-   **Path Parameter:** `id` (integer, required) - The unique ID of the drug.
-   **Description:** Retrieves a single drug by its ID.
-   **Example:** `GET /api/drugs/1`
-   **Response:** A single drug object or a `404 Not Found` error.

## 🤝 Contributing

Contributions are what make the open-source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

1.  **Fork the Project**
2.  **Create your Feature Branch** (`git checkout -b feature/AmazingFeature`)
3.  **Commit your Changes** (`git commit -m 'Add some AmazingFeature'`)
4.  **Push to the Branch** (`git push origin feature/AmazingFeature`)
5.  **Open a Pull Request**

Please make sure to update tests as appropriate and ensure the code is formatted with `cargo fmt`.

## 📜 License

Distributed under the MIT License. See `LICENSE` file for more information.

---