use std::sync::Arc;

use axum::{Router, routing::get};
use tokio::sync::broadcast;

use crate::api::handlers::{create_order, health};
use crate::config::AppConfig;
use crate::db::repository::OrderRepository;
use crate::ws::price_stream::PriceTick;

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub repository: Arc<OrderRepository>,
    pub price_tx: broadcast::Sender<PriceTick>,
}

impl AppState {
    pub fn new(
        config: AppConfig,
        repository: Arc<OrderRepository>,
        price_tx: broadcast::Sender<PriceTick>,
    ) -> Self {
        Self {
            config,
            repository,
            price_tx,
        }
    }
}

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/orders", axum::routing::post(create_order))
        .with_state(state)
}
