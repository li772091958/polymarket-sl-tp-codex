use async_trait::async_trait;

use crate::models::order::Order;

#[async_trait]
pub trait OrderExecutor: Send + Sync {
    async fn execute_market_order(&self, order: &Order) -> anyhow::Result<()>;
}
