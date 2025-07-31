use std::sync::Arc;
use tokio_tungstenite::{self, tungstenite::Message};
use serde_json;
use futures_util::{SinkExt, StreamExt}; // extended trait for ws_stream.split()
use tokio::{self, sync::*};
use std::time::Duration;
use async_trait::async_trait;

use crate::ws::*;

#[async_trait]
pub trait GatewayEventHandler: Send + Sync {
    
}

// Main Discord Gateway struct
pub struct DiscordGateway {
    pub url: Option<String>,
    pub token: String,
    ws: Arc<Mutex<MyWebSocket>>,

    heartbeat_interval: Option<tokio::task::JoinHandle<()>>,
    handler: Option<Arc<dyn GatewayEventHandler>>
}

impl DiscordGateway {
    pub async fn connect(token: &str, mut url: Option<&str>) -> anyhow::Result<Self> {
        if url.is_none() {
            url = Some("wss://gateway.discord.gg/?v=10&encoding=json");
        }

        let ws = MyWebSocket::new(url.unwrap()).await?;

        Ok(
            Self {
                token: token.to_string(),
                url: Some(url.unwrap().to_string()),
                ws: Arc::new(Mutex::new(ws)),
                heartbeat_interval: None,
                handler: None,
            }
        )
    }

    // Send a single heartbeat message
    async fn send_heartbeat(&mut self) -> anyhow::Result<()> {
        let mut ws = self.ws.lock().await;
        ws.send(Message::Text("{\"op\":1,\"d\":null}".to_string().into())).await
            .map_err(|e| anyhow::anyhow!("Failed to send heartbeat: {}", e))
    }

    // Start the heartbeat loop in a separate task
    pub async fn start_heart_beat(&mut self, ms: Duration) -> anyhow::Result<bool> {
        // Check if heartbeat is already running
        {
            if self.heartbeat_interval.is_some() {
                return Err(anyhow::anyhow!("Heartbeat is already running."));
            }
        }

        // Send the first heartbeat
        self.send_heartbeat().await?;
        
        let ws = self.ws.clone();
        
        // Spawn a task that sends heartbeat messages at regular intervals
        let heartbeat_task = tokio::spawn(async move {
            let mut interval = tokio::time::interval(ms);
            
            loop {
                interval.tick().await;
                
                // Send the heartbeat message
                let heartbeat_msg = Message::Text("{\"op\":1,\"d\":null}".to_string().into());
                let mut ws = ws.lock().await;
                if ws.send(heartbeat_msg).await.is_err() {
                    eprintln!("Failed to send heartbeat");
                    break;
                }
            }
        });
        
        // Store the task handle for later cleanup
        self.heartbeat_interval = Some(heartbeat_task);

        Ok(true)
    }
    
    ///
    pub async fn stop_heart_beat(&mut self) -> anyhow::Result<bool> {
        // Check if heartbeat is running
        if self.heartbeat_interval.is_none() {
            return Err(anyhow::anyhow!("Heartbeat is not running."));
        }
        
        // Abort the heartbeat task
        if let Some(task) = self.heartbeat_interval.take() {
            task.abort();
            // let _ = task.await;
        }
        
        Ok(true)
    }



}