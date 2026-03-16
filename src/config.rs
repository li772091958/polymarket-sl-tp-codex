use anyhow::Context;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub database_url: String,
    pub polymarket_ws_url: String,
    pub order_execution_timeout_ms: u64,
    pub http_bind: String,
}

impl AppConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        let database_url =
            std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://data/bot.db".to_string());
        let polymarket_ws_url = std::env::var("POLYMARKET_WS_URL")
            .unwrap_or_else(|_| "wss://ws-subscriptions-clob.polymarket.com/ws/market".to_string());
        let order_execution_timeout_ms = std::env::var("ORDER_EXECUTION_TIMEOUT_MS")
            .unwrap_or_else(|_| "40".to_string())
            .parse::<u64>()
            .context("parse ORDER_EXECUTION_TIMEOUT_MS")?;
        let http_bind = std::env::var("HTTP_BIND").unwrap_or_else(|_| "0.0.0.0:8080".to_string());

        Ok(Self {
            database_url,
            polymarket_ws_url,
            order_execution_timeout_ms,
            http_bind,
        })
    }
}
