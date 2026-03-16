use axum::{Json, extract::State};
use chrono::Utc;
use tracing::info;
use uuid::Uuid;

use crate::api::dto::{CreateOrderRequest, CreateOrderResponse};
use crate::app::AppState;
use crate::error::AppError;
use crate::models::order::{Order, OrderStatus};

pub async fn health() -> &'static str {
    "ok"
}

pub async fn create_order(
    State(state): State<AppState>,
    Json(req): Json<CreateOrderRequest>,
) -> Result<Json<CreateOrderResponse>, AppError> {
    if req.sl.is_none() && req.tp.is_none() {
        return Err(AppError::BadRequest(
            "at least one of sl or tp must be provided".to_string(),
        ));
    }

    if req.size <= 0.0 {
        return Err(AppError::BadRequest("size must be > 0".to_string()));
    }

    let now = Utc::now();
    let order = Order {
        id: Uuid::new_v4().to_string(),
        token_id: req.token_id,
        size: req.size,
        side: req.side,
        sl: req.sl,
        tp: req.tp,
        status: OrderStatus::Monitoring,
        trigger_reason: None,
        trigger_price: None,
        created_at: now,
        updated_at: now,
        executed_at: None,
    };

    state
        .repository
        .insert(&order)
        .await
        .map_err(AppError::from)?;
    info!(order_id = %order.id, token_id = %order.token_id, "SL/TP order registered");

    Ok(Json(CreateOrderResponse {
        id: order.id,
        status: "monitoring",
    }))
}
