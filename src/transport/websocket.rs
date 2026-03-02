// transport/websocket.rs

use tokio_tungstenite::connect_async;
use futures_util::StreamExt;
use url::Url;

pub async fn connect(url: &str) -> anyhow::Result<tokio_tungstenite::WebSocketStream<
    tokio_tungstenite::tungstenite::stream::MaybeTlsStream<tokio::net::TcpStream>
>> {
    let (ws_stream, _) = connect_async(Url::parse(url)?).await?;
    Ok(ws_stream)
}