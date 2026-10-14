use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct AddressRecord {
    pub id: Uuid,
    pub user_id: Uuid,
    pub label: String,
    pub recipient_name: String,
    pub recipient_phone: String,
    pub full_address: String,
    pub additional_details: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub is_primary: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct CreateAddress<'a> {
    pub user_id: Uuid,
    pub label: Option<&'a str>,
    pub recipient_name: &'a str,
    pub recipient_phone: &'a str,
    pub full_address: &'a str,
    pub additional_details: Option<&'a str>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub is_primary: bool,
}

pub struct UpdateAddress<'a> {
    pub user_id: Uuid,
    pub address_id: Uuid,
    pub label: Option<&'a str>,
    pub recipient_name: Option<&'a str>,
    pub recipient_phone: Option<&'a str>,
    pub full_address: Option<&'a str>,
    pub additional_details: Option<&'a str>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub is_primary: Option<bool>,
}

pub async fn list_by_user(pool: &PgPool, user_id: Uuid) -> Result<Vec<AddressRecord>, sqlx::Error> {
    sqlx::query_as::<_, AddressRecord>(
        r#"
        SELECT id, user_id, label, recipient_name, recipient_phone, full_address, additional_details,
               latitude, longitude, is_primary, created_at, updated_at
        FROM addresses
        WHERE user_id = $1
        ORDER BY is_primary DESC, updated_at DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

pub async fn find_by_id_and_user(
    pool: &PgPool,
    address_id: Uuid,
    user_id: Uuid,
) -> Result<Option<AddressRecord>, sqlx::Error> {
    sqlx::query_as::<_, AddressRecord>(
        r#"
        SELECT id, user_id, label, recipient_name, recipient_phone, full_address, additional_details,
               latitude, longitude, is_primary, created_at, updated_at
        FROM addresses
        WHERE id = $1 AND user_id = $2
        LIMIT 1
        "#,
    )
    .bind(address_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await
}

pub async fn create(pool: &PgPool, input: CreateAddress<'_>) -> Result<AddressRecord, sqlx::Error> {
    let CreateAddress {
        user_id,
        label,
        recipient_name,
        recipient_phone,
        full_address,
        additional_details,
        latitude,
        longitude,
        is_primary,
    } = input;
    let mut tx = pool.begin().await?;

    if is_primary {
        sqlx::query("UPDATE addresses SET is_primary = FALSE WHERE user_id = $1")
            .bind(user_id)
            .execute(&mut *tx)
            .await?;
    }

    let record = sqlx::query_as::<_, AddressRecord>(
        r#"
        INSERT INTO addresses (
            user_id, label, recipient_name, recipient_phone, full_address, additional_details,
            latitude, longitude, is_primary
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING id, user_id, label, recipient_name, recipient_phone, full_address, additional_details,
                  latitude, longitude, is_primary, created_at, updated_at
        "#,
    )
    .bind(user_id)
    .bind(label.unwrap_or("Rumah"))
    .bind(recipient_name)
    .bind(recipient_phone)
    .bind(full_address)
    .bind(additional_details)
    .bind(latitude)
    .bind(longitude)
    .bind(is_primary)
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(record)
}

pub async fn update(
    pool: &PgPool,
    input: UpdateAddress<'_>,
) -> Result<Option<AddressRecord>, sqlx::Error> {
    let UpdateAddress {
        user_id,
        address_id,
        label,
        recipient_name,
        recipient_phone,
        full_address,
        additional_details,
        latitude,
        longitude,
        is_primary,
    } = input;
    let existing = find_by_id_and_user(pool, address_id, user_id).await?;
    let Some(mut record) = existing else {
        return Ok(None);
    };

    let mut tx = pool.begin().await?;

    if matches!(is_primary, Some(true)) {
        sqlx::query("UPDATE addresses SET is_primary = FALSE WHERE user_id = $1 AND id != $2")
            .bind(user_id)
            .bind(address_id)
            .execute(&mut *tx)
            .await?;
    }

    let next_label = label
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(record.label.as_str());
    let next_recipient_name = recipient_name
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(record.recipient_name.as_str());
    let next_recipient_phone = recipient_phone
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(record.recipient_phone.as_str());
    let next_full_address = full_address
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(record.full_address.as_str());
    let next_additional_details = additional_details
        .map(str::trim)
        .filter(|value| !value.is_empty());
    let next_latitude = latitude.or(record.latitude);
    let next_longitude = longitude.or(record.longitude);
    let next_is_primary = is_primary.unwrap_or(record.is_primary);

    record = sqlx::query_as::<_, AddressRecord>(
        r#"
        UPDATE addresses
        SET label = $3,
            recipient_name = $4,
            recipient_phone = $5,
            full_address = $6,
            additional_details = $7,
            latitude = $8,
            longitude = $9,
            is_primary = $10,
            updated_at = NOW()
        WHERE id = $1 AND user_id = $2
        RETURNING id, user_id, label, recipient_name, recipient_phone, full_address, additional_details,
                  latitude, longitude, is_primary, created_at, updated_at
        "#,
    )
    .bind(address_id)
    .bind(user_id)
    .bind(next_label)
    .bind(next_recipient_name)
    .bind(next_recipient_phone)
    .bind(next_full_address)
    .bind(next_additional_details)
    .bind(next_latitude)
    .bind(next_longitude)
    .bind(next_is_primary)
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(Some(record))
}

pub async fn delete_by_id(
    pool: &PgPool,
    user_id: Uuid,
    address_id: Uuid,
) -> Result<bool, sqlx::Error> {
    let rows = sqlx::query(
        r#"
        DELETE FROM addresses
        WHERE id = $1 AND user_id = $2
        "#,
    )
    .bind(address_id)
    .bind(user_id)
    .execute(pool)
    .await?;

    Ok(rows.rows_affected() > 0)
}
