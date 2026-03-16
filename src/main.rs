use std::sync::Arc;

use anyhow::Context;
use tokio::net::TcpListener;
use tracing::info;

use polymarket_sl_tp_bot::app::AppState;
use polymarket_sl_tp_bot::clob::client::PolymarketClobClient;
use polymarket_sl_tp_bot::config::AppConfig;
use polymarket_sl_tp_bot::db::repository::OrderRepository;
use polymarket_sl_tp_bot::db::sqlite::{connect_sqlite, run_migrations};
use polymarket_sl_tp_bot::engine::monitor::start_trigger_engine;
use polymarket_sl_tp_bot::logging::init_tracing;
use polymarket_sl_tp_bot::ws::polymarket_ws::start_ws_listener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();

    let config = AppConfig::from_env()?;
    let pool = connect_sqlite(&config.database_url).await?;
    run_migrations(&pool).await?;

    let (price_tx, _price_rx) = tokio::sync::broadcast::channel(4096);
    let repository = Arc::new(OrderRepository::new(pool.clone()));
    let clob_client = Arc::new(PolymarketClobClient::new(config.clone()));

    start_ws_listener(config.clone(), price_tx.clone());
    start_trigger_engine(repository.clone(), clob_client, price_tx.subscribe());

    let state = AppState::new(config, repository, price_tx);
    let app = polymarket_sl_tp_bot::app::build_router(state);

    let listener = TcpListener::bind(&state.config.http_bind)
        .await
        .context("bind HTTP listener")?;
    info!(bind = %state.config.http_bind, "HTTP server listening");
    axum::serve(listener, app).await.context("start axum")?;

    Ok(())
}
