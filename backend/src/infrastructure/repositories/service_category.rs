use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone, Debug, sqlx::FromRow)]
pub struct ServiceCategoryRow {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub base_price_idr: i64,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub async fn list_active(pool: &PgPool) -> Result<Vec<ServiceCategoryRow>, sqlx::Error> {
    sqlx::query_as::<_, ServiceCategoryRow>(
        r#"
        SELECT id, name, description, base_price_idr, is_active, created_at, updated_at
        FROM service_categories
        WHERE is_active = true
        ORDER BY name ASC
        "#,
    )
    .fetch_all(pool)
    .await
}

pub async fn find_by_id(
    pool: &PgPool,
    service_id: Uuid,
) -> Result<Option<ServiceCategoryRow>, sqlx::Error> {
    sqlx::query_as::<_, ServiceCategoryRow>(
        r#"
        SELECT id, name, description, base_price_idr, is_active, created_at, updated_at
        FROM service_categories
        WHERE id = $1 AND is_active = true
        LIMIT 1
        "#,
    )
    .bind(service_id)
    .fetch_optional(pool)
    .await
}
