use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct WorkerProfileRecord {
    pub id: Uuid,
    pub user_id: Uuid,
    pub bio: Option<String>,
    pub is_available: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub async fn find_by_user_id(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Option<WorkerProfileRecord>, sqlx::Error> {
    sqlx::query_as::<_, WorkerProfileRecord>(
        r#"
        SELECT id, user_id, bio, is_available, created_at, updated_at
        FROM worker_profiles
        WHERE user_id = $1
        LIMIT 1
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
}

pub async fn upsert_profile(
    pool: &PgPool,
    user_id: Uuid,
    bio: Option<&str>,
    is_available: bool,
) -> Result<WorkerProfileRecord, sqlx::Error> {
    sqlx::query_as::<_, WorkerProfileRecord>(
        r#"
        INSERT INTO worker_profiles (user_id, bio, is_available)
        VALUES ($1, $2, $3)
        ON CONFLICT (user_id) DO UPDATE SET
            bio = EXCLUDED.bio,
            is_available = EXCLUDED.is_available,
            updated_at = NOW()
        RETURNING id, user_id, bio, is_available, created_at, updated_at
        "#,
    )
    .bind(user_id)
    .bind(bio)
    .bind(is_available)
    .fetch_one(pool)
    .await
}
