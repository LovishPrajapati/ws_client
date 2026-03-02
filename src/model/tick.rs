
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct Tick {
    pub symbol: String,
    pub price: f64,
    pub quantity: f64,
    pub trade_id: u64,
    pub side: AggressorSide,
    pub exchange_ts: u64,
    pub received_ts: SystemTime
}

#[derive(Debug, Clone, Copy)]
enum AggressorSide {
    Buy,
    Sell
}
