use anyhow::Context;
use chrono::{DateTime, Utc};
use sqlx::{Row, SqlitePool};

use crate::models::order::{Order, OrderStatus, Side};

#[derive(Clone)]
pub struct OrderRepository {
    pool: SqlitePool,
}

impl OrderRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn insert(&self, order: &Order) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO orders
            (id, token_id, size, side, sl, tp, status, trigger_reason, trigger_price, created_at, updated_at, executed_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)"#,
        )
        .bind(&order.id)
        .bind(&order.token_id)
        .bind(order.size)
        .bind(format_side(order.side))
        .bind(order.sl)
        .bind(order.tp)
        .bind(format_status(order.status))
        .bind(&order.trigger_reason)
        .bind(order.trigger_price)
        .bind(order.created_at.to_rfc3339())
        .bind(order.updated_at.to_rfc3339())
        .bind(order.executed_at.map(|v| v.to_rfc3339()))
        .execute(&self.pool)
        .await
        .context("insert order")?;

        Ok(())
    }

    pub async fn list_monitoring_by_token(&self, token_id: &str) -> anyhow::Result<Vec<Order>> {
        let rows = sqlx::query(
            "SELECT id, token_id, size, side, sl, tp, status, trigger_reason, trigger_price, created_at, updated_at, executed_at FROM orders WHERE token_id = ?1 AND status = 'monitoring'",
        )
        .bind(token_id)
        .fetch_all(&self.pool)
        .await
        .context("list monitoring by token")?;

        rows.into_iter().map(row_to_order).collect()
    }

    pub async fn mark_triggered(
        &self,
        id: &str,
        reason: &str,
        trigger_price: f64,
        executed_at: DateTime<Utc>,
    ) -> anyhow::Result<()> {
        sqlx::query(
            "UPDATE orders SET status='triggered', trigger_reason=?2, trigger_price=?3, executed_at=?4, updated_at=?5 WHERE id=?1",
        )
        .bind(id)
        .bind(reason)
        .bind(trigger_price)
        .bind(executed_at.to_rfc3339())
        .bind(Utc::now().to_rfc3339())
        .execute(&self.pool)
        .await
        .context("mark triggered")?;

        Ok(())
    }

    pub async fn mark_failed(&self, id: &str, reason: &str) -> anyhow::Result<()> {
        sqlx::query(
            "UPDATE orders SET status='failed', trigger_reason=?2, updated_at=?3 WHERE id=?1",
        )
        .bind(id)
        .bind(reason)
        .bind(Utc::now().to_rfc3339())
        .execute(&self.pool)
        .await
        .context("mark failed")?;

        Ok(())
    }
}

fn row_to_order(row: sqlx::sqlite::SqliteRow) -> anyhow::Result<Order> {
    Ok(Order {
        id: row.try_get("id")?,
        token_id: row.try_get("token_id")?,
        size: row.try_get("size")?,
        side: parse_side(&row.try_get::<String, _>("side")?)?,
        sl: row.try_get("sl")?,
        tp: row.try_get("tp")?,
        status: parse_status(&row.try_get::<String, _>("status")?)?,
        trigger_reason: row.try_get("trigger_reason")?,
        trigger_price: row.try_get("trigger_price")?,
        created_at: DateTime::parse_from_rfc3339(&row.try_get::<String, _>("created_at")?)?
            .with_timezone(&Utc),
        updated_at: DateTime::parse_from_rfc3339(&row.try_get::<String, _>("updated_at")?)?
            .with_timezone(&Utc),
        executed_at: row
            .try_get::<Option<String>, _>("executed_at")?
            .map(|v| DateTime::parse_from_rfc3339(&v).map(|dt| dt.with_timezone(&Utc)))
            .transpose()?,
    })
}

fn format_side(side: Side) -> &'static str {
    match side {
        Side::Buy => "BUY",
        Side::Sell => "SELL",
    }
}

fn parse_side(side: &str) -> anyhow::Result<Side> {
    match side {
        "BUY" => Ok(Side::Buy),
        "SELL" => Ok(Side::Sell),
        _ => anyhow::bail!("invalid side: {side}"),
    }
}

fn format_status(status: OrderStatus) -> &'static str {
    match status {
        OrderStatus::Monitoring => "monitoring",
        OrderStatus::Triggered => "triggered",
        OrderStatus::Failed => "failed",
    }
}

fn parse_status(status: &str) -> anyhow::Result<OrderStatus> {
    match status {
        "monitoring" => Ok(OrderStatus::Monitoring),
        "triggered" => Ok(OrderStatus::Triggered),
        "failed" => Ok(OrderStatus::Failed),
        _ => anyhow::bail!("invalid status: {status}"),
    }
}
