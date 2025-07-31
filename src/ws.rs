use std::sync::Arc;
use async_trait::async_trait;
use tokio_tungstenite::{self, connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};
use futures_util::{stream::{SplitSink, SplitStream}, SinkExt, StreamExt}; // extended trait for ws_stream.split()
use tokio::{self, net::TcpStream, sync::*};

#[async_trait]
pub trait WsEventHandler: Send + Sync {
    /// Handle a web socket message.
    async fn on_message(&self, msg: Message) -> () {}

    /// Handle a web socket open event.
    async fn on_open(&self) -> () {}

    /// Handle a web socket close event.
    async fn on_close(&self, code: u64, reason: String) -> () {}

    /// Handle a error from web socket
    async fn on_error(&self, code: u64, reason: String) -> () {}
}

pub struct MyWebSocket {
    writer: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
    reader: Arc<Mutex<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>>,
}

impl MyWebSocket {
    pub async fn new(url: &str) -> anyhow::Result<Self> {
        let (ws_stream, _) = connect_async(url).await.unwrap();

        let (write, read) = ws_stream.split();

        let writer = Arc::new(Mutex::new(write));
        let reader = Arc::new(Mutex::new(read));

        Ok(
            Self {
                writer,
                reader,
            }
        )
    }

    pub async fn send(&mut self, msg: Message) -> anyhow::Result<()> {
        let mut writer = self.writer.lock().await;
        writer.send(msg).await.map_err(|e| anyhow::anyhow!("Failed to send message: {}", e))
    }

    pub async fn recv(&mut self) -> anyhow::Result<Message> {
        let mut reader = self.reader.lock().await;
        Ok(reader.next().await.unwrap()?)
    }

    pub async fn close(&mut self) -> anyhow::Result<()> {
        let mut writer = self.writer.lock().await;
        writer.close().await.map_err(|e| anyhow::anyhow!("Failed to close WebSocket: {}", e))
    }

    pub async fn event_handler(&mut self, handler: Arc<dyn WsEventHandler>) {
        let reader = self.reader.clone();
        
        tokio::spawn(async move {
            let mut reader = reader.lock().await;
            
            while let Some(result) = reader.next().await {
                match result {
                    Ok(msg) => {
                        handler.on_message(msg).await;
                    }
                    Err(e) => {
                        handler.on_error(0, e.to_string()).await;
                    }
                }
            }

            handler.on_close(1000, "normal closure".to_string()).await;
        });
    }
}