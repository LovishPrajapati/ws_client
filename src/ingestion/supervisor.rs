// ingestion/supervisor.rs

use tokio::sync::mpsc;
use crate::producer::task::start_producer;
use crate::exchange::binance::BinanceParser;
use crate::model::tick::Tick;

pub async fn start_system() {

    let (tx, mut rx) = mpsc::channel::<Tick>(200_000);

    // Producer 1
    tokio::spawn(start_producer(
        "wss://stream.binance.com:9443/ws/btcusdt@aggTrade".into(),
        Box::new(BinanceParser),
        tx.clone(),
    ));

    // Producer 2
    tokio::spawn(start_producer(
        "wss://stream.binance.com:9443/ws/ethusdt@aggTrade".into(),
        Box::new(BinanceParser),
        tx.clone(),
    ));

    // Aggregator
    tokio::spawn(async move {
        while let Some(tick) = rx.recv().await {
            println!("{:?}", tick);
        }
    });
}