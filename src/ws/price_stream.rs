use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceTick {
    pub token_id: String,
    pub price: f64,
    pub ts_millis: i64,
}
