use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserRecord {
    pub id: Uuid,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub password_hash: Option<String>,
    pub role: String,
    pub display_name: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub async fn find_by_id(pool: &PgPool, user_id: Uuid) -> Result<Option<UserRecord>, sqlx::Error> {
    sqlx::query_as::<_, UserRecord>(
        r#"
        SELECT id, email, phone, password_hash, role, display_name, is_active, created_at, updated_at
        FROM users
        WHERE id = $1
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
}

pub async fn find_by_email_or_phone(
    pool: &PgPool,
    email: Option<&str>,
    phone: Option<&str>,
) -> Result<Option<UserRecord>, sqlx::Error> {
    let email = email.filter(|value| !value.trim().is_empty());
    let phone = phone.filter(|value| !value.trim().is_empty());

    match (email, phone) {
        (Some(email), None) => {
            sqlx::query_as::<_, UserRecord>(
                r#"
                SELECT id, email, phone, password_hash, role, display_name, is_active, created_at, updated_at
                FROM users
                WHERE email = $1
                LIMIT 1
                "#,
            )
            .bind(email)
            .fetch_optional(pool)
            .await
        }
        (None, Some(phone)) => {
            sqlx::query_as::<_, UserRecord>(
                r#"
                SELECT id, email, phone, password_hash, role, display_name, is_active, created_at, updated_at
                FROM users
                WHERE phone = $1
                LIMIT 1
                "#,
            )
            .bind(phone)
            .fetch_optional(pool)
            .await
        }
        (Some(email), Some(phone)) => {
            sqlx::query_as::<_, UserRecord>(
                r#"
                SELECT id, email, phone, password_hash, role, display_name, is_active, created_at, updated_at
                FROM users
                WHERE email = $1 OR phone = $2
                LIMIT 1
                "#,
            )
            .bind(email)
            .bind(phone)
            .fetch_optional(pool)
            .await
        }
        (None, None) => Ok(None),
    }
}

pub async fn create_user(
    pool: &PgPool,
    email: Option<&str>,
    phone: Option<&str>,
    password_hash: &str,
    role: &str,
    display_name: &str,
) -> Result<UserRecord, sqlx::Error> {
    sqlx::query_as::<_, UserRecord>(
        r#"
        INSERT INTO users (email, phone, password_hash, role, display_name, is_active)
        VALUES ($1, $2, $3, $4, $5, TRUE)
        RETURNING id, email, phone, password_hash, role, display_name, is_active, created_at, updated_at
        "#,
    )
    .bind(email)
    .bind(phone)
    .bind(password_hash)
    .bind(role)
    .bind(display_name)
    .fetch_one(pool)
    .await
}

pub async fn update_profile(
    pool: &PgPool,
    user_id: Uuid,
    display_name: Option<&str>,
    phone: Option<&str>,
) -> Result<UserRecord, sqlx::Error> {
    let updated_name = display_name
        .map(|name| name.trim())
        .filter(|name| !name.is_empty());
    let updated_phone = phone
        .map(|value| value.trim())
        .filter(|value| !value.is_empty());

    let row = sqlx::query_as::<_, UserRecord>(
        r#"
        UPDATE users
        SET display_name = COALESCE($2, display_name),
            phone = COALESCE($3, phone),
            updated_at = NOW()
        WHERE id = $1
        RETURNING id, email, phone, password_hash, role, display_name, is_active, created_at, updated_at
        "#,
    )
    .bind(user_id)
    .bind(updated_name)
    .bind(updated_phone)
    .fetch_one(pool)
    .await?;

    Ok(row)
}
