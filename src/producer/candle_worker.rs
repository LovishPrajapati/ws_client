use tokio::sync::mpsc;
use crate::model::event::CandleEvent;
use crate::analytics::candle_aggregator::CandleAggregator;

pub async fn start_candle_worker(
    _symbol: String,
    mut rx: mpsc::Receiver<CandleEvent>,
) {
    let mut agg_1m  = CandleAggregator::new(60_000);
    let mut agg_5m  = CandleAggregator::new(300_000);
    let mut agg_15m = CandleAggregator::new(900_000);
    let mut agg_1h  = CandleAggregator::new(3_600_000);

    while let Some(event) = rx.recv().await {
        match event {
            CandleEvent::Tick(tick) => {
                if let Some(c1) = agg_1m.update(&tick) {

                    println!("1m candle closed {:?}", c1);

                    if let Some(c5) = agg_5m.update_from_candle(&c1) {
                        println!("5m candle closed {:?}", c5);
                    }

                    if let Some(c15) = agg_15m.update_from_candle(&c1) {
                        println!("15m candle closed {:?}", c15);
                    }

                    if let Some(c1h) = agg_1h.update_from_candle(&c1) {
                        println!("1h candle closed {:?}", c1h);
                    }
                }
            }

            CandleEvent::MinuteClose(ts) => {

                if let Some(c1) = agg_1m.force_close(ts) {

                    println!("1m closed by timer {:?}", c1);

                    if let Some(c5) = agg_5m.update_from_candle(&c1) {
                        println!("5m closed {:?}", c5);
                    }

                    if let Some(c15) = agg_15m.update_from_candle(&c1) {
                        println!("15m closed {:?}", c15);
                    }

                    if let Some(c1h) = agg_1h.update_from_candle(&c1) {
                        println!("1h closed {:?}", c1h);
                    }
                }
            }
        }
    }
}