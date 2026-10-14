use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct OrderRecord {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub worker_id: Option<Uuid>,
    pub service_category_id: Uuid,
    pub service_price_idr: i64,
    pub customer_fee_idr: i64,
    pub worker_fee_idr: i64,
    pub status: String,
    pub address_id: Option<Uuid>,
    pub address: Option<String>,
    pub notes: Option<String>,
    pub accepted_at: Option<DateTime<Utc>>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub cancelled_at: Option<DateTime<Utc>>,
    pub cancellation_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct CreateOrder<'a> {
    pub customer_id: Uuid,
    pub service_category_id: Uuid,
    pub address_id: Uuid,
    pub address_snapshot: &'a str,
    pub notes: Option<&'a str>,
    pub service_price_idr: i64,
    pub customer_fee_idr: i64,
    pub worker_fee_idr: i64,
}

pub async fn create_order(
    pool: &PgPool,
    input: CreateOrder<'_>,
) -> Result<OrderRecord, sqlx::Error> {
    let CreateOrder {
        customer_id,
        service_category_id,
        address_id,
        address_snapshot,
        notes,
        service_price_idr,
        customer_fee_idr,
        worker_fee_idr,
    } = input;
    sqlx::query_as::<_, OrderRecord>(
        r#"
        INSERT INTO orders (
            customer_id,
            worker_id,
            service_category_id,
            service_price_idr,
            customer_fee_idr,
            worker_fee_idr,
            status,
            address_id,
            address,
            notes
        )
        VALUES ($1, NULL, $2, $3, $4, $5, 'pending', $6, $7, $8)
        RETURNING
            id, customer_id, worker_id, service_category_id, service_price_idr,
            customer_fee_idr, worker_fee_idr, status, address_id, address, notes,
            accepted_at, started_at, completed_at, cancelled_at, cancellation_reason,
            created_at, updated_at
        "#,
    )
    .bind(customer_id)
    .bind(service_category_id)
    .bind(service_price_idr)
    .bind(customer_fee_idr)
    .bind(worker_fee_idr)
    .bind(address_id)
    .bind(address_snapshot)
    .bind(notes)
    .fetch_one(pool)
    .await
}

pub async fn find_by_id(pool: &PgPool, order_id: Uuid) -> Result<Option<OrderRecord>, sqlx::Error> {
    sqlx::query_as::<_, OrderRecord>(
        r#"
        SELECT
            id, customer_id, worker_id, service_category_id, service_price_idr,
            customer_fee_idr, worker_fee_idr, status, address_id, address, notes,
            accepted_at, started_at, completed_at, cancelled_at, cancellation_reason,
            created_at, updated_at
        FROM orders
        WHERE id = $1
        LIMIT 1
        "#,
    )
    .bind(order_id)
    .fetch_optional(pool)
    .await
}

pub async fn list_for_customer(
    pool: &PgPool,
    customer_id: Uuid,
    limit: i32,
    offset: i32,
) -> Result<Vec<OrderRecord>, sqlx::Error> {
    sqlx::query_as::<_, OrderRecord>(
        r#"
        SELECT
            id, customer_id, worker_id, service_category_id, service_price_idr,
            customer_fee_idr, worker_fee_idr, status, address_id, address, notes,
            accepted_at, started_at, completed_at, cancelled_at, cancellation_reason,
            created_at, updated_at
        FROM orders
        WHERE customer_id = $1
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(customer_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
}

pub async fn list_available_for_worker(
    pool: &PgPool,
    limit: i32,
    offset: i32,
) -> Result<Vec<OrderRecord>, sqlx::Error> {
    sqlx::query_as::<_, OrderRecord>(
        r#"
        SELECT
            id, customer_id, worker_id, service_category_id, service_price_idr,
            customer_fee_idr, worker_fee_idr, status, address_id, address, notes,
            accepted_at, started_at, completed_at, cancelled_at, cancellation_reason,
            created_at, updated_at
        FROM orders
        WHERE status = 'pending' AND worker_id IS NULL
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
}

pub async fn list_for_worker(
    pool: &PgPool,
    worker_id: Uuid,
    limit: i32,
    offset: i32,
) -> Result<Vec<OrderRecord>, sqlx::Error> {
    sqlx::query_as::<_, OrderRecord>(
        r#"
        SELECT
            id, customer_id, worker_id, service_category_id, service_price_idr,
            customer_fee_idr, worker_fee_idr, status, address_id, address, notes,
            accepted_at, started_at, completed_at, cancelled_at, cancellation_reason,
            created_at, updated_at
        FROM orders
        WHERE worker_id = $1
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(worker_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
}

pub async fn accept_order(
    pool: &PgPool,
    order_id: Uuid,
    worker_id: Uuid,
) -> Result<Option<OrderRecord>, sqlx::Error> {
    let mut tx = pool.begin().await?;
    let row = sqlx::query_as::<_, OrderRecord>(
        r#"
        SELECT
            id, customer_id, worker_id, service_category_id, service_price_idr,
            customer_fee_idr, worker_fee_idr, status, address_id, address, notes,
            accepted_at, started_at, completed_at, cancelled_at, cancellation_reason,
            created_at, updated_at
        FROM orders
        WHERE id = $1
        FOR UPDATE
        "#,
    )
    .bind(order_id)
    .fetch_optional(&mut *tx)
    .await?;

    let Some(mut order) = row else {
        tx.rollback().await?;
        return Ok(None);
    };

    if order.status != "pending" || order.worker_id.is_some() {
        tx.rollback().await?;
        return Ok(Some(order));
    }

    order = sqlx::query_as::<_, OrderRecord>(
        r#"
        UPDATE orders
        SET worker_id = $2,
            status = 'accepted',
            accepted_at = NOW(),
            updated_at = NOW()
        WHERE id = $1
        RETURNING
            id, customer_id, worker_id, service_category_id, service_price_idr,
            customer_fee_idr, worker_fee_idr, status, address_id, address, notes,
            accepted_at, started_at, completed_at, cancelled_at, cancellation_reason,
            created_at, updated_at
        "#,
    )
    .bind(order_id)
    .bind(worker_id)
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(Some(order))
}

pub async fn start_order(
    pool: &PgPool,
    order_id: Uuid,
    worker_id: Uuid,
) -> Result<Option<OrderRecord>, sqlx::Error> {
    let mut tx = pool.begin().await?;
    let order = sqlx::query_as::<_, OrderRecord>(
        r#"
        SELECT
            id, customer_id, worker_id, service_category_id, service_price_idr,
            customer_fee_idr, worker_fee_idr, status, address_id, address, notes,
            accepted_at, started_at, completed_at, cancelled_at, cancellation_reason,
            created_at, updated_at
        FROM orders
        WHERE id = $1 AND worker_id = $2
        FOR UPDATE
        "#,
    )
    .bind(order_id)
    .bind(worker_id)
    .fetch_optional(&mut *tx)
    .await?;

    let Some(current) = order else {
        tx.rollback().await?;
        return Ok(None);
    };

    if current.status != "accepted" {
        tx.rollback().await?;
        return Ok(Some(current));
    }

    let updated = sqlx::query_as::<_, OrderRecord>(
        r#"
        UPDATE orders
        SET status = 'in_progress',
            started_at = NOW(),
            updated_at = NOW()
        WHERE id = $1 AND worker_id = $2
        RETURNING
            id, customer_id, worker_id, service_category_id, service_price_idr,
            customer_fee_idr, worker_fee_idr, status, address_id, address, notes,
            accepted_at, started_at, completed_at, cancelled_at, cancellation_reason,
            created_at, updated_at
        "#,
    )
    .bind(order_id)
    .bind(worker_id)
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(Some(updated))
}

pub async fn complete_order(
    pool: &PgPool,
    order_id: Uuid,
    worker_id: Uuid,
) -> Result<Option<OrderRecord>, sqlx::Error> {
    let mut tx = pool.begin().await?;
    let order = sqlx::query_as::<_, OrderRecord>(
        r#"
        SELECT
            id, customer_id, worker_id, service_category_id, service_price_idr,
            customer_fee_idr, worker_fee_idr, status, address_id, address, notes,
            accepted_at, started_at, completed_at, cancelled_at, cancellation_reason,
            created_at, updated_at
        FROM orders
        WHERE id = $1 AND worker_id = $2
        FOR UPDATE
        "#,
    )
    .bind(order_id)
    .bind(worker_id)
    .fetch_optional(&mut *tx)
    .await?;

    let Some(current) = order else {
        tx.rollback().await?;
        return Ok(None);
    };

    if current.status != "in_progress" {
        tx.rollback().await?;
        return Ok(Some(current));
    }

    let updated = sqlx::query_as::<_, OrderRecord>(
        r#"
        UPDATE orders
        SET status = 'completed',
            completed_at = NOW(),
            updated_at = NOW()
        WHERE id = $1 AND worker_id = $2
        RETURNING
            id, customer_id, worker_id, service_category_id, service_price_idr,
            customer_fee_idr, worker_fee_idr, status, address_id, address, notes,
            accepted_at, started_at, completed_at, cancelled_at, cancellation_reason,
            created_at, updated_at
        "#,
    )
    .bind(order_id)
    .bind(worker_id)
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(Some(updated))
}

pub async fn cancel_order(
    pool: &PgPool,
    order_id: Uuid,
    user_id: Uuid,
    role: &str,
    reason: Option<&str>,
) -> Result<Option<OrderRecord>, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let order = sqlx::query_as::<_, OrderRecord>(
        r#"
        SELECT
            id, customer_id, worker_id, service_category_id, service_price_idr,
            customer_fee_idr, worker_fee_idr, status, address_id, address, notes,
            accepted_at, started_at, completed_at, cancelled_at, cancellation_reason,
            created_at, updated_at
        FROM orders
        WHERE id = $1
        FOR UPDATE
        "#,
    )
    .bind(order_id)
    .fetch_optional(&mut *tx)
    .await?;

    let Some(current) = order else {
        tx.rollback().await?;
        return Ok(None);
    };

    let allowed = match role {
        "customer" => {
            current.customer_id == user_id
                && matches!(current.status.as_str(), "pending" | "accepted")
        }
        "worker" => {
            current.worker_id == Some(user_id)
                && matches!(current.status.as_str(), "accepted" | "in_progress")
        }
        _ => false,
    };

    if !allowed || matches!(current.status.as_str(), "completed" | "cancelled") {
        tx.rollback().await?;
        return Ok(Some(current));
    }

    let updated = sqlx::query_as::<_, OrderRecord>(
        r#"
        UPDATE orders
        SET status = 'cancelled',
            cancelled_at = NOW(),
            cancellation_reason = $2,
            updated_at = NOW()
        WHERE id = $1
        RETURNING
            id, customer_id, worker_id, service_category_id, service_price_idr,
            customer_fee_idr, worker_fee_idr, status, address_id, address, notes,
            accepted_at, started_at, completed_at, cancelled_at, cancellation_reason,
            created_at, updated_at
        "#,
    )
    .bind(order_id)
    .bind(reason)
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(Some(updated))
}
