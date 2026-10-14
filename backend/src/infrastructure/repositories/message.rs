use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MessageRecord {
    pub id: Uuid,
    pub conversation_id: Uuid,
    pub sender_id: Uuid,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub async fn list_for_order(
    pool: &PgPool,
    order_id: Uuid,
    limit: i32,
    offset: i32,
) -> Result<Vec<MessageRecord>, sqlx::Error> {
    sqlx::query_as::<_, MessageRecord>(
        r#"
        SELECT m.id, m.conversation_id, m.sender_id, m.body, m.created_at, m.updated_at
        FROM messages m
        JOIN order_conversations c ON c.id = m.conversation_id
        WHERE c.order_id = $1
        ORDER BY m.created_at ASC, m.id ASC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(order_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
}

pub async fn create(
    pool: &PgPool,
    conversation_id: Uuid,
    sender_id: Uuid,
    body: &str,
) -> Result<MessageRecord, sqlx::Error> {
    sqlx::query_as::<_, MessageRecord>(
        r#"
        INSERT INTO messages (conversation_id, sender_id, body)
        VALUES ($1, $2, $3)
        RETURNING id, conversation_id, sender_id, body, created_at, updated_at
        "#,
    )
    .bind(conversation_id)
    .bind(sender_id)
    .bind(body)
    .fetch_one(pool)
    .await
}
