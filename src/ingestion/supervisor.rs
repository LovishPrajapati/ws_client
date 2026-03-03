// ingestion/supervisor.rs

use std::sync::Arc;
use tokio::sync::{broadcast, mpsc};
use crate::producer::task::start_producer;
use crate::exchange::binance::BinanceParser;
use crate::model::tick::Tick;

pub async fn start_system() {

    let (tx, mut rx) = mpsc::channel::<Tick>(200_000);
    let (shutdown_tx, _) = broadcast::channel::<()>(1);

    let shutdown_rx = shutdown_tx.subscribe();
    let shutdown_rx1 = shutdown_tx.subscribe();
    // Producer 1
    tokio::spawn(start_producer(
        "wss://stream.binance.com:9443/ws/btcusdt@aggTrade".into(),
        Box::new(BinanceParser),
        tx.clone(),
        shutdown_rx
    ));

    // Producer 2
    tokio::spawn(start_producer(
        "wss://stream.binance.com:9443/ws/ethusdt@aggTrade".into(),
        Box::new(BinanceParser),
        tx.clone(),
        shutdown_rx1
    ));

    // Aggregator
    tokio::spawn(async move {
        while let Some(tick) = rx.recv().await {
            println!("{:?}", tick);
        }
    });
}