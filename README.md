# polymarket-sl-tp-bot

A low-latency Rust bot that adds Stop Loss (SL) / Take Profit (TP) capability for Polymarket orders.

## Features
- Axum HTTP API to register SL/TP watches after trade fill.
- Tokio websocket listener for live price stream.
- Trigger engine with multi-token concurrent monitoring.
- SQLite persistence via SQLx.
- Structured logs via tracing.

## Quick Start
1. Copy `.env.example` to `.env`.
2. Create DB: run migrations with SQLx CLI or your preferred SQLite tool.
3. Start service: `cargo run`.

## API
`POST /orders`

```json
{
  "token_id": "<token-id>",
  "size": 10.0,
  "side": "BUY",
  "sl": 0.35,
  "tp": 0.55
}
```
