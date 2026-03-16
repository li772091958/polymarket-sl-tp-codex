use crate::models::order::{Order, Side};
use crate::models::trigger::TriggerReason;

pub fn should_trigger(order: &Order, price: f64) -> Option<TriggerReason> {
    match order.side {
        Side::Buy => {
            if let Some(sl) = order.sl {
                if price <= sl {
                    return Some(TriggerReason::StopLoss);
                }
            }
            if let Some(tp) = order.tp {
                if price >= tp {
                    return Some(TriggerReason::TakeProfit);
                }
            }
        }
        Side::Sell => {
            if let Some(sl) = order.sl {
                if price >= sl {
                    return Some(TriggerReason::StopLoss);
                }
            }
            if let Some(tp) = order.tp {
                if price <= tp {
                    return Some(TriggerReason::TakeProfit);
                }
            }
        }
    }

    None
}
