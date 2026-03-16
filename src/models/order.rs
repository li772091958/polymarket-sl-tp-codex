use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    Monitoring,
    Triggered,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: String,
    pub token_id: String,
    pub size: f64,
    pub side: Side,
    pub sl: Option<f64>,
    pub tp: Option<f64>,
    pub status: OrderStatus,
    pub trigger_reason: Option<String>,
    pub trigger_price: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub executed_at: Option<DateTime<Utc>>,
}
