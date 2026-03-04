use crate::model::tick::Tick;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use rdkafka::ClientConfig;
use tokio::sync::mpsc;

pub async fn start_kafka_producer(mut rx: mpsc::Receiver<Tick>) {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .set("linger.ms", "5")
        .set("compression.type", "lz4")
        .create()
        .expect("Kafka producer error");

    let topic = "market_ticks";

    while let Some(tick) = rx.recv().await {
        let payload = serde_json::to_string(&tick).unwrap();

        let _ = producer
            .send(
                FutureRecord::to(topic).payload(&payload).key(&tick.symbol),
                Timeout::Never,
            )
            .await;
    }
}
