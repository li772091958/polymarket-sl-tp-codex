use std::time::Duration;

use futures_util::StreamExt;
use tokio::sync::broadcast;
use tokio_tungstenite::connect_async;
use tracing::{error, info, warn};

use crate::config::AppConfig;
use crate::ws::price_stream::PriceTick;

#[derive(serde::Deserialize)]
struct WsMessage {
    token_id: String,
    price: f64,
    #[serde(default)]
    ts_millis: Option<i64>,
}

pub fn start_ws_listener(config: AppConfig, tx: broadcast::Sender<PriceTick>) {
    tokio::spawn(async move {
        loop {
            match connect_async(&config.polymarket_ws_url).await {
                Ok((stream, _)) => {
                    info!(url = %config.polymarket_ws_url, "connected to polymarket websocket");
                    let (_, mut read) = stream.split();

                    while let Some(msg) = read.next().await {
                        match msg {
                            Ok(frame) if frame.is_text() => {
                                let text = frame.into_text().unwrap_or_default();
                                match serde_json::from_str::<WsMessage>(&text) {
                                    Ok(parsed) => {
                                        let tick = PriceTick {
                                            token_id: parsed.token_id,
                                            price: parsed.price,
                                            ts_millis: parsed.ts_millis.unwrap_or_else(|| {
                                                chrono::Utc::now().timestamp_millis()
                                            }),
                                        };
                                        let _ = tx.send(tick);
                                    }
                                    Err(err) => {
                                        warn!(error=%err, payload=%text, "ignored unrecognized ws payload");
                                    }
                                }
                            }
                            Ok(_) => {}
                            Err(err) => {
                                warn!(error=%err, "websocket frame error");
                                break;
                            }
                        }
                    }
                }
                Err(err) => {
                    error!(error = %err, "failed connecting websocket");
                }
            }

            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });
}
