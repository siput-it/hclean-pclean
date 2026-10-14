use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ConversationRecord {
    pub id: Uuid,
    pub order_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub async fn find_by_order_id(
    pool: &PgPool,
    order_id: Uuid,
) -> Result<Option<ConversationRecord>, sqlx::Error> {
    sqlx::query_as::<_, ConversationRecord>(
        r#"
        SELECT id, order_id, created_at, updated_at
        FROM order_conversations
        WHERE order_id = $1
        LIMIT 1
        "#,
    )
    .bind(order_id)
    .fetch_optional(pool)
    .await
}

pub async fn ensure_for_order(
    pool: &PgPool,
    order_id: Uuid,
) -> Result<ConversationRecord, sqlx::Error> {
    if let Some(existing) = find_by_order_id(pool, order_id).await? {
        return Ok(existing);
    }

    sqlx::query_as::<_, ConversationRecord>(
        r#"
        INSERT INTO order_conversations (order_id)
        VALUES ($1)
        RETURNING id, order_id, created_at, updated_at
        "#,
    )
    .bind(order_id)
    .fetch_one(pool)
    .await
}

pub async fn list_for_user(
    pool: &PgPool,
    user_id: Uuid,
    limit: i32,
    offset: i32,
) -> Result<Vec<ConversationRecord>, sqlx::Error> {
    sqlx::query_as::<_, ConversationRecord>(
        r#"
        SELECT c.id, c.order_id, c.created_at, c.updated_at
        FROM order_conversations c
        JOIN orders o ON o.id = c.order_id
        WHERE o.customer_id = $1 OR o.worker_id = $1
        ORDER BY c.updated_at DESC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(user_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
}
