use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct NotificationRecord {
    pub id: Uuid,
    pub user_id: Uuid,
    pub type_: String,
    pub title: String,
    pub body: String,
    pub resource_type: Option<String>,
    pub resource_id: Option<Uuid>,
    pub read_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

pub async fn create_for_user(
    pool: &PgPool,
    user_id: Uuid,
    type_: &str,
    title: &str,
    body: &str,
    resource_type: Option<&str>,
    resource_id: Option<Uuid>,
) -> Result<NotificationRecord, sqlx::Error> {
    sqlx::query_as::<_, NotificationRecord>(
        r#"
        INSERT INTO notifications (user_id, type, title, body, resource_type, resource_id)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, user_id, type AS type_, title, body, resource_type, resource_id, read_at, created_at
        "#,
    )
    .bind(user_id)
    .bind(type_)
    .bind(title)
    .bind(body)
    .bind(resource_type)
    .bind(resource_id)
    .fetch_one(pool)
    .await
}

pub async fn list_for_user(
    pool: &PgPool,
    user_id: Uuid,
    limit: i32,
    offset: i32,
) -> Result<Vec<NotificationRecord>, sqlx::Error> {
    sqlx::query_as::<_, NotificationRecord>(
        r#"
        SELECT id, user_id, type AS type_, title, body, resource_type, resource_id, read_at, created_at
        FROM notifications
        WHERE user_id = $1
        ORDER BY created_at DESC, id DESC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(user_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
}

pub async fn unread_count(pool: &PgPool, user_id: Uuid) -> Result<i64, sqlx::Error> {
    let row: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*)
        FROM notifications
        WHERE user_id = $1 AND read_at IS NULL
        "#,
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(row.0)
}

pub async fn mark_read(
    pool: &PgPool,
    user_id: Uuid,
    notification_id: Uuid,
) -> Result<Option<NotificationRecord>, sqlx::Error> {
    sqlx::query_as::<_, NotificationRecord>(
        r#"
        UPDATE notifications
        SET read_at = NOW()
        WHERE user_id = $1 AND id = $2 AND read_at IS NULL
        RETURNING id, user_id, type AS type_, title, body, resource_type, resource_id, read_at, created_at
        "#,
    )
    .bind(user_id)
    .bind(notification_id)
    .fetch_optional(pool)
    .await
}

pub async fn mark_all_read(pool: &PgPool, user_id: Uuid) -> Result<i64, sqlx::Error> {
    let result = sqlx::query(
        r#"
        UPDATE notifications
        SET read_at = NOW()
        WHERE user_id = $1 AND read_at IS NULL
        "#,
    )
    .bind(user_id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() as i64)
}
