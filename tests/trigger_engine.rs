use polymarket_sl_tp_bot::engine::trigger_eval::should_trigger;
use polymarket_sl_tp_bot::models::order::{Order, OrderStatus, Side};

#[test]
fn buy_tp_triggers_when_price_above_threshold() {
    let now = chrono::Utc::now();
    let order = Order {
        id: "1".into(),
        token_id: "t1".into(),
        size: 1.0,
        side: Side::Buy,
        sl: Some(0.3),
        tp: Some(0.5),
        status: OrderStatus::Monitoring,
        trigger_reason: None,
        trigger_price: None,
        created_at: now,
        updated_at: now,
        executed_at: None,
    };

    assert!(should_trigger(&order, 0.51).is_some());
}
