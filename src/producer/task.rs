// producer/task.rs

use crate::exchange::traits::ExchangeParser;
use crate::model::tick::Tick;
use crate::transport::websocket::connect;
use futures_util::StreamExt;
use tokio::sync::mpsc;

pub async fn start_producer(
    url: String,
    parser: Box<dyn ExchangeParser>,
    tx: mpsc::Sender<Tick>,
    mut shutdown: tokio::sync::broadcast::Receiver<()>,
) {
    loop {
        tokio::select! {
            _ = shutdown.recv() => {
                println!("Shutting down producer");
                return;
            }

            result = connect(&url) => {
                match result {
                    Ok(ws_stream) => {
                        let (_, mut read) = ws_stream.split();
                        loop {
                            tokio::select! {
                                _ = shutdown.recv() => {
                                    println!("Shutting down producer");
                                    return;
                                }

                                msg = read.next() => {
                                    match msg {
                                        Some(Ok(msg)) if msg.is_text() => {
                                            if let Ok(Some(tick)) =
                                                parser.parse(msg.to_text().unwrap())
                                            {
                                                if tx.send(tick).await.is_err() {
                                                    return;
                                                }
                                            }
                                        }
                                        Some(Err(_)) => break,
                                        None => break,
                                        _ => {}
                                    }
                                }
                            }
                        }
                    }

                    Err(e) => {
                        eprintln!("Reconnect error: {:?}", e);

                        tokio::select! {
                            _ = shutdown.recv() => {
                                println!("Shutting down producer");
                                return;
                            }
                            _ = tokio::time::sleep(std::time::Duration::from_secs(2)) => {}
                        }
                    }
                }
            }
        }
    }
}
