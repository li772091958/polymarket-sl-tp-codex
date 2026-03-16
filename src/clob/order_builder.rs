use crate::models::order::Order;

pub struct MarketOrderRequest {
    pub token_id: String,
    pub size: f64,
    pub side: String,
}

pub fn build_market_order(order: &Order) -> MarketOrderRequest {
    MarketOrderRequest {
        token_id: order.token_id.clone(),
        size: order.size,
        side: match order.side {
            crate::models::order::Side::Buy => "BUY".to_string(),
            crate::models::order::Side::Sell => "SELL".to_string(),
        },
    }
}
