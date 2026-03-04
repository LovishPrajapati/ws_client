use crate::model::{candle::Candle, tick::Tick};

pub struct CandleAggregator {
    timeframe_ms: u64,
    current: Option<Candle>,
}

impl CandleAggregator {
    pub fn new(timeframe_ms: u64) -> Self {
        Self {
            current: None,
            timeframe_ms,
        }
    }

    pub fn update(&mut self, tick: &Tick) -> Option<Candle> {
        let bucket = (tick.exchange_ts / self.timeframe_ms) * self.timeframe_ms;

        match &mut self.current {
            Some(c) if c.open_time == bucket => {
                c.high = c.high.max(tick.price);
                c.low = c.low.min(tick.price);
                c.close = tick.price;
                c.volume += tick.quantity * tick.price;
                None
            }

            Some(c) => {
                let closed = c.clone();

                self.current = Some(Candle {
                    symbol: tick.symbol.clone(),
                    open_time: bucket,
                    open: tick.price,
                    high: tick.price,
                    low: tick.price,
                    close: tick.price,
                    volume: tick.quantity * tick.price,
                });

                Some(closed)
            }

            None => {
                self.current = Some(Candle {
                    symbol: tick.symbol.clone(),
                    open_time: bucket,
                    open: tick.price,
                    high: tick.price,
                    low: tick.price,
                    close: tick.price,
                    volume: tick.quantity * tick.price,
                });

                None
            }
        }
    }

    pub fn force_close(&mut self, ts: u64) -> Option<Candle> {
        if let Some(c) = &self.current {
            if c.open_time + self.timeframe_ms <= ts {
                let closed = self.current.take();
                return closed;
            }
        }

        None
    }

    pub fn update_from_candle(&mut self, c: &Candle) -> Option<Candle> {
        let bucket = (c.open_time / self.timeframe_ms) * self.timeframe_ms;

        match &mut self.current {
            Some(curr) if curr.open_time == bucket => {
                curr.high = curr.high.max(c.high);
                curr.low = curr.low.min(c.low);
                curr.close = c.close;
                curr.volume += c.volume;

                None
            }

            Some(curr) => {
                let closed = curr.clone();

                self.current = Some(Candle {
                    symbol: c.symbol.clone(),
                    open_time: bucket,
                    open: c.open,
                    high: c.high,
                    low: c.low,
                    close: c.close,
                    volume: c.volume,
                });

                Some(closed)
            }

            None => {
                self.current = Some(Candle {
                    symbol: c.symbol.clone(),
                    open_time: bucket,
                    open: c.open,
                    high: c.high,
                    low: c.low,
                    close: c.close,
                    volume: c.volume,
                });

                None
            }
        }
    }
}
