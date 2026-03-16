#!/usr/bin/env bash
set -euo pipefail
mkdir -p data
[ -f data/bot.db ] || ./scripts/init_db.sh
RUST_LOG=${RUST_LOG:-info,polymarket_sl_tp_bot=debug} cargo run
