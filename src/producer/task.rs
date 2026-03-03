// producer/task.rs

use tokio::sync::mpsc;
use futures_util::StreamExt;
use crate::model::tick::Tick;
use crate::exchange::traits::ExchangeParser;
use crate::transport::websocket::connect;

pub async fn start_producer(
    url: String,
    parser: Box<dyn ExchangeParser>,
    tx: mpsc::Sender<Tick>,
) {

    loop {
        match connect(&url).await {
            Ok(ws_stream) => {
                let (_, mut read) = ws_stream.split();

                while let Some(msg) = read.next().await {
                    if let Ok(msg) = msg {
                        if msg.is_text() {
                            if let Ok(Some(tick)) =
                                parser.parse(msg.to_text().unwrap())
                            {
                                if tx.send(tick).await.is_err() {
                                    return;
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Reconnect error: {:?}", e);
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            }
        }
    }
}