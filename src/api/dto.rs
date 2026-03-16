use serde::{Deserialize, Serialize};

use crate::models::order::Side;

#[derive(Debug, Deserialize)]
pub struct CreateOrderRequest {
    pub token_id: String,
    pub size: f64,
    pub side: Side,
    pub sl: Option<f64>,
    pub tp: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct CreateOrderResponse {
    pub id: String,
    pub status: &'static str,
}
