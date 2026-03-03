// src/main.rs

mod model;
mod exchange;
mod transport;
mod producer;
mod ingestion;

use tokio::sync::mpsc;
use tokio::signal;
use tracing::info;
use tracing_subscriber;

use crate::model::tick::Tick;
use crate::producer::task::start_producer;
use crate::exchange::binance::BinanceParser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize structured logging
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    info!("Starting Market Data Ingestion System");

    // Bounded channel → defines backpressure
    let (tx, mut rx) = mpsc::channel::<Tick>(200_000);

    // ==========================
    // Spawn Producers
    // ==========================

    let producer_urls = vec![
        "wss://stream.binance.com:9443/ws/btcusdt@aggTrade",
        "wss://stream.binance.com:9443/ws/ethusdt@aggTrade",
    ];

    for url in producer_urls {
        let tx_clone = tx.clone();
        let parser = Box::new(BinanceParser);

        tokio::spawn(async move {
            start_producer(url.to_string(), parser, tx_clone).await;
        });
    }

    drop(tx); // Important: allow channel to close cleanly

    // ==========================
    // Aggregator Task
    // ==========================

    let aggregator_handle = tokio::spawn(async move {
        let mut total_ticks: u64 = 0;

        while let Some(tick) = rx.recv().await {
            total_ticks += 1;

            // Replace this with:
            // - Time-series engine
            // - Router
            // - Kafka writer
            // - Analytics engine
            info!("{:?}", tick);

            if total_ticks % 10_000 == 0 {
                info!("Processed {} ticks", total_ticks);
            }
        }

        info!("Aggregator shutting down");
    });

    // ==========================
    // Graceful Shutdown
    // ==========================

    tokio::select! {
        _ = signal::ctrl_c() => {
            info!("Shutdown signal received");
        }
    }

    info!("Waiting for aggregator to finish...");
    aggregator_handle.await?;

    info!("System terminated cleanly");

    Ok(())
}