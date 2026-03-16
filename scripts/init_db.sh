#!/usr/bin/env bash
set -euo pipefail
sqlite3 data/bot.db < migrations/0001_init_orders.sql
sqlite3 data/bot.db < migrations/0002_add_indexes.sql
echo "DB initialized at data/bot.db"
