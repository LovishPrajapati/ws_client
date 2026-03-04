use std::time::SystemTime;
use tokio::sync::mpsc;
use crate::model::{tick::Tick, candle::Candle};
use crate::model::event::CandleEvent;
use chrono::Utc;

pub async fn start_candle_worker(
    symbol: String,
    mut rx: mpsc::Receiver<CandleEvent>,
) {
    let mut current: Option<Candle> = None;

    while let Some(event) = rx.recv().await {

        match event {

            CandleEvent::Tick(tick) => {

                let bucket = (tick.exchange_ts / 60_000) * 60_000;

                match &mut current {

                    Some(c) if c.open_time == bucket => {
                        c.high = c.high.max(tick.price);
                        c.low = c.low.min(tick.price);
                        c.close = tick.price;
                        c.volume += tick.quantity * tick.price;
                    }

                    Some(c) => {
                        println!("CLOSED: {:?}", c);

                        current = Some(Candle {
                            symbol: symbol.clone(),
                            open_time: bucket,
                            open: tick.price,
                            high: tick.price,
                            low: tick.price,
                            close: tick.price,
                            volume: tick.quantity * tick.price,
                        });
                    }

                    None => {
                        current = Some(Candle {
                            symbol: symbol.clone(),
                            open_time: bucket,
                            open: tick.price,
                            high: tick.price,
                            low: tick.price,
                            close: tick.price,
                            volume: tick.quantity * tick.price,
                        });
                    }
                }
            }

            CandleEvent::MinuteClose(minute) => {

                if let Some(c) = &current {
                    if c.open_time < minute {
                        println!("CLOSED BY TIMER:{:?} {:?}", Utc::now() ,c);
                        current = None;
                    }
                }

            }
        }
    }
}