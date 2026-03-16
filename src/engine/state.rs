use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub struct MonitorState {
    pub active_tokens: HashSet<String>,
    pub last_price_by_token: HashMap<String, f64>,
}
