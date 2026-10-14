use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub access_token_hash: String,
    pub refresh_token_hash: String,
    pub session_status: String,
    pub expires_at: DateTime<Utc>,
    pub refresh_expires_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub async fn create_session(
    pool: &PgPool,
    user_id: Uuid,
    access_token_hash: &str,
    refresh_token_hash: &str,
    expires_at: DateTime<Utc>,
    refresh_expires_at: DateTime<Utc>,
) -> Result<UserSession, sqlx::Error> {
    sqlx::query_as::<_, UserSession>(
        r#"
        INSERT INTO user_sessions (
            user_id,
            access_token_hash,
            refresh_token_hash,
            session_status,
            expires_at,
            refresh_expires_at,
            revoked_at
        )
        VALUES ($1, $2, $3, 'active', $4, $5, NULL)
        RETURNING id, user_id, access_token_hash, refresh_token_hash, session_status, expires_at, refresh_expires_at, revoked_at, created_at, updated_at
        "#,
    )
    .bind(user_id)
    .bind(access_token_hash)
    .bind(refresh_token_hash)
    .bind(expires_at)
    .bind(refresh_expires_at)
    .fetch_one(pool)
    .await
}

pub async fn revoke_all_for_user(pool: &PgPool, user_id: Uuid) -> Result<u64, sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE user_sessions
        SET session_status = 'revoked',
            revoked_at = NOW(),
            updated_at = NOW()
        WHERE user_id = $1 AND session_status = 'active'
        "#,
    )
    .bind(user_id)
    .execute(pool)
    .await
    .map(|result| result.rows_affected())
}

pub async fn revoke_with_access_hash(
    pool: &PgPool,
    access_token_hash: &str,
) -> Result<u64, sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE user_sessions
        SET session_status = 'revoked',
            revoked_at = NOW(),
            updated_at = NOW()
        WHERE access_token_hash = $1 AND session_status = 'active'
        "#,
    )
    .bind(access_token_hash)
    .execute(pool)
    .await
    .map(|result| result.rows_affected())
}

pub async fn find_active_session_for_user(
    pool: &PgPool,
    user_id: Uuid,
    access_token_hash: &str,
) -> Result<Option<UserSession>, sqlx::Error> {
    sqlx::query_as::<_, UserSession>(
        r#"
        SELECT id, user_id, access_token_hash, refresh_token_hash, session_status, expires_at, refresh_expires_at, revoked_at, created_at, updated_at
        FROM user_sessions
        WHERE user_id = $1
          AND access_token_hash = $2
          AND session_status = 'active'
          AND expires_at > NOW()
        LIMIT 1
        "#,
    )
    .bind(user_id)
    .bind(access_token_hash)
    .fetch_optional(pool)
    .await
}
