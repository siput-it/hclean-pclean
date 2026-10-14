use std::time::Duration;

use sqlx::{postgres::PgPoolOptions, PgPool};

async fn connect_if_configured() -> Option<PgPool> {
    let database_url = match std::env::var("DATABASE_URL") {
        Ok(value) if !value.trim().is_empty() => value,
        _ => return None,
    };

    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&database_url)
        .await
        .ok()
}

#[tokio::test]
async fn database_bootstraps_service_catalog_when_available() {
    let pool = match connect_if_configured().await {
        Some(pool) => pool,
        None => {
            eprintln!("DATABASE_URL not configured; skipping database integration test");
            return;
        }
    };

    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM service_categories")
        .fetch_one(&pool)
        .await
        .expect("service_categories harus ada setelah migrasi");

    assert!(count >= 7, "minimal set layanan harus ter-seed: {count}");
}
