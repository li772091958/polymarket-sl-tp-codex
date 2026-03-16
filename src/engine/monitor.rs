use std::sync::Arc;

use chrono::Utc;
use tokio::sync::broadcast;
use tracing::{debug, error, info, warn};

use crate::clob::client::PolymarketClobClient;
use crate::db::repository::OrderRepository;
use crate::engine::executor::OrderExecutor;
use crate::engine::trigger_eval::should_trigger;
use crate::ws::price_stream::PriceTick;

pub fn start_trigger_engine(
    repository: Arc<OrderRepository>,
    executor: Arc<PolymarketClobClient>,
    mut rx: broadcast::Receiver<PriceTick>,
) {
    tokio::spawn(async move {
        info!("trigger engine started");
        loop {
            match rx.recv().await {
                Ok(tick) => {
                    let start = std::time::Instant::now();
                    if let Err(err) = process_tick(&repository, &*executor, tick).await {
                        error!(error = %err, "failed processing tick");
                    }
                    let elapsed_ms = start.elapsed().as_micros() as f64 / 1000.0;
                    if elapsed_ms > 50.0 {
                        warn!(
                            latency_ms = elapsed_ms,
                            "trigger pipeline latency exceeded target"
                        );
                    } else {
                        debug!(latency_ms = elapsed_ms, "trigger pipeline latency");
                    }
                }
                Err(broadcast::error::RecvError::Lagged(skipped)) => {
                    warn!(skipped, "trigger engine lagged on price channel");
                }
                Err(broadcast::error::RecvError::Closed) => break,
            }
        }
    });
}

async fn process_tick(
    repository: &OrderRepository,
    executor: &dyn OrderExecutor,
    tick: PriceTick,
) -> anyhow::Result<()> {
    let orders = repository.list_monitoring_by_token(&tick.token_id).await?;
    for order in orders {
        if let Some(reason) = should_trigger(&order, tick.price) {
            let result = executor.execute_market_order(&order).await;
            match result {
                Ok(()) => {
                    repository
                        .mark_triggered(&order.id, reason.as_str(), tick.price, Utc::now())
                        .await?;
                    info!(order_id = %order.id, token_id = %order.token_id, price = tick.price, reason = reason.as_str(), "order triggered and executed");
                }
                Err(err) => {
                    repository
                        .mark_failed(&order.id, &format!("execution_failed: {err}"))
                        .await?;
                    error!(order_id = %order.id, error = %err, "order execution failed");
                }
            }
        }
    }
    Ok(())
}
