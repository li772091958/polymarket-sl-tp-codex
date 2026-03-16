use std::time::Duration;

use async_trait::async_trait;
use tokio::time::timeout;
use tracing::info;

use crate::clob::order_builder::build_market_order;
use crate::config::AppConfig;
use crate::engine::executor::OrderExecutor;
use crate::models::order::Order;

// Ensure official SDK is part of this binary crate.
use rs_clob_client as _;

pub struct PolymarketClobClient {
    config: AppConfig,
}

impl PolymarketClobClient {
    pub fn new(config: AppConfig) -> Self {
        Self { config }
    }
}

#[async_trait]
impl OrderExecutor for PolymarketClobClient {
    async fn execute_market_order(&self, order: &Order) -> anyhow::Result<()> {
        let request = build_market_order(order);

        timeout(
            Duration::from_millis(self.config.order_execution_timeout_ms),
            async {
                // TODO: replace this with authenticated official SDK order placement.
                info!(
                    token_id = %request.token_id,
                    side = %request.side,
                    size = request.size,
                    "simulated market execution via clob client"
                );
                Ok::<(), anyhow::Error>(())
            },
        )
        .await
        .map_err(|_| anyhow::anyhow!("execution timed out"))??;

        Ok(())
    }
}
