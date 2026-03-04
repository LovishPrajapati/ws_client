#[derive(Debug, Clone)]
pub struct Candle {
    pub symbol: String,
    pub open_time: u64,
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub volume: f64,
}
