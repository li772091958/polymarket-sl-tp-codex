#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriggerReason {
    StopLoss,
    TakeProfit,
}

impl TriggerReason {
    pub fn as_str(self) -> &'static str {
        match self {
            TriggerReason::StopLoss => "stop_loss",
            TriggerReason::TakeProfit => "take_profit",
        }
    }
}
