pub fn round_price(v: f64) -> f64 {
    (v * 1_000_000.0).round() / 1_000_000.0
}
