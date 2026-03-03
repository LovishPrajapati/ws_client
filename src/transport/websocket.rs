// transport/websocket.rs

use tokio_tungstenite::connect_async;
use url::Url;

/// Connect to a WebSocket URL. Returns the async WebSocketStream from tokio-tungstenite
/// (which uses tokio_tungstenite::MaybeTlsStream, not tungstenite's sync MaybeTlsStream).
pub async fn connect(
    url: &str,
) -> anyhow::Result<
    tokio_tungstenite::WebSocketStream<
        tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
    >,
> {
    let (ws_stream, _) = connect_async(Url::parse(url)?).await?;
    Ok(ws_stream)
}