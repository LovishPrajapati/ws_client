// src/main.rs

mod model;
mod exchange;
mod transport;
mod producer;

use tokio::sync::{mpsc, broadcast};
use tokio::signal;
use tracing::info;
use tracing_subscriber;
use std::collections::HashMap;
use std::time::{Duration, SystemTime,UNIX_EPOCH};
use tokio::time::interval;
use crate::model::tick::Tick;
use crate::producer::task::start_producer;
use crate::exchange::binance::BinanceParser;
use crate::model::event::CandleEvent;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize structured logging
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();
    let symbols = vec![
        "btcusdt",
        "dogeusdt",
        "dotusdt",
        "arbusdt",
        "bnbusdt",
        "edusdt",
        "solusdt",
        "suiusdt",
        "xrpusdt",
        "linkusdt",
        "aaveusdt",
        "ldousdt",
        "bchusdt",
        "crvusdt",
        "ltcusdt",
        "atomusdt",
        "sandusdt",
        "opusdt",
        "uniusdt",
        "wldusdt",
        "trbusdt",
        "storjusdt",
        "pythusdt",
        "apeusdt",
        "cyberusdt",
        "kavausdt",
        "seiusdt",
        "spellusdt",
        "adausdt",
        "zecusdt",
        "joeusdt",
        "avaxusdt",
        "aptusdt",
        "trxusdt",
        "nearusdt",
        "1000bonkusdt",
        "1000pepeusdt",
        "1000shibusdt",
        "filusdt",
        "api3usdt",
        "icpusdt",
        "rdntusdt",
        "minausdt",
        "mavusdt",
        "sklusdt",
        "injusdt",
        "movrusdt",
        "aliceusdt",
        "axsusdt",
        "bandusdt",
        "batusdt",
        "blurusdt",
        "roseusdt",
        "snxusdt",
        "gmtusdt",
        "stxusdt",
        "sushiusdt",
        "dydxusdt",
        "runeusdt",
        "galausdt",
        "yggusdt",
        "etcusdt",
        "bicousdt",
        "chzusdt",
        "tiausdt",
        "ordiusdt",
        "pendleusdt",
        "fetusdt",
        "woousdt",
        "enjusdt",
        "superusdt",
        "arkusdt",
        "altusdt",
        "lskusdt",
        "jtousdt",
        "wifusdt",
        "mantausdt",
        "xaiusdt",
        "umausdt",
        "ondousdt",
        "jupusdt",
        "1000satsusdt",
        "compusdt",
        "egldusdt",
        "tonusdt",
        "algousdt",
        "gmxusdt",
        "cotiusdt",
        "zetausdt",
        "strkusdt",
        "dymusdt",
        "roninusdt",
        "portalusdt",
        "cfxusdt",
        "imxusdt",
        "pixelusdt",
        "beamxusdt",
        "1000flokiusdt",
        "memeusdt",
        "maskusdt",
        "manausdt",
        "peopleusdt",
        "arkmusdt",
        "bomeusdt",
        "arusdt",
        "bigtimeusdt",
        "zrxusdt",
        "polyxusdt",
        "grtusdt",
        "ensusdt",
        "thetausdt",
        "xlmusdt",
        "rsrusdt",
        "aceusdt",
        "nfpusdt",
        "aiusdt",
        "kasusdt",
        "idusdt",
        "1000luncusdt",
        "hbarusdt",
        "zilusdt",
        "aevousdt",
        "glmusdt",
        "metisusdt",
        "axlusdt",
        "ethfiusdt",
        "vanryusdt",
        "renderusdt",
        "notusdt",
        "popcatusdt",
        "taousdt",
        "turbousdt",
        "brettusdt",
        "mewusdt",
        "enausdt",
        "zrousdt",
        "c98usdt",
        "lrcusdt",
    ];

    info!("Starting Market Data Ingestion System");

    // Bounded channel → defines backpressure
    let (tx, mut rx) = mpsc::channel::<Tick>(200_000);
    let (shutdown_tx, _) = broadcast::channel::<()>(1);

    // ==========================
    // Spawn Producers
    // ==========================

    let mut producer_urls = Vec::with_capacity(symbols.len());

    for symbol in symbols{
        producer_urls.push(format!("wss://stream.binance.com:9443/ws/{0}@aggTrade", symbol));
    }

    for url in producer_urls {
        let tx_clone = tx.clone();
        let parser = Box::new(BinanceParser);
        let shutdown_rx = shutdown_tx.subscribe();

        tokio::spawn(async move {
            start_producer(url.to_string(), parser, tx_clone,shutdown_rx).await;
        });
    }

    drop(tx); // Important: allow channel to close cleanly

    let router_handle = tokio::spawn(async move {

        let mut symbol_workers: HashMap<String, mpsc::Sender<CandleEvent>> = HashMap::new();

        let mut timer = interval(Duration::from_secs(1));

        loop {
            tokio::select! {
                _ = timer.tick() => {
                     let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64;

                let minute_close = (now / 60_000) * 60_000;

                for tx in symbol_workers.values() {
                    let _ = tx.send(CandleEvent::MinuteClose(minute_close)).await;
                }
                }

                Some(tick) = rx.recv() => {
                    let symbol = tick.symbol.clone();
                    let entry = symbol_workers.entry(symbol.clone());

                    let symbol_tx = entry.or_insert_with(|| {

                    let (tx_symbol, rx_symbol) = mpsc::channel::<CandleEvent>(10_000);
                    let symbol = tick.symbol.clone();

                    tokio::spawn(async move {
                        producer::candle_worker::start_candle_worker(symbol, rx_symbol).await;
                    });

                    tx_symbol
                });
             let _ = symbol_tx.send(CandleEvent::Tick(tick)).await;

            }
            }
        }
    });

    // ==========================
    // Graceful Shutdown
    // ==========================

    tokio::select! {
        _ = signal::ctrl_c() => {
            info!("Shutdown signal received");
             shutdown_tx.send(()).ok();
        }
    }

    // info!("Waiting for aggregator to finish...");
    // aggregator_handle.await?;
    router_handle.await?;

    info!("System terminated cleanly");

    Ok(())
}