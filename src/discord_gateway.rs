use std::sync::Arc;
use tokio_tungstenite::{self, tungstenite::Message};
use serde_json;
use tokio::{self, sync::*};
use std::time::Duration;
use async_trait::async_trait;

use crate::ws::*;

#[async_trait]
pub trait GatewayEventHandler: Send + Sync {
    async fn hello(&self, msg: Message) -> () {}
    async fn ready(&self, msg: Message) -> () {}
    async fn resumed(&self, msg: Message) -> () {}
    async fn reconnect(&self, msg: Message) -> () {}
    async fn invalid_session(&self, msg: Message) -> () {}
    async fn application_command_permissions_update(&self, msg: Message) -> () {}
    async fn auto_moderation_rule_create(&self, msg: Message) -> () {}
    async fn auto_moderation_rule_update(&self, msg: Message) -> () {}
    async fn auto_moderation_rule_delete(&self, msg: Message) -> () {}
    async fn auto_moderation_action_execution(&self, msg: Message) -> () {}
    async fn channel_create(&self, msg: Message) -> () {}
    async fn channel_update(&self, msg: Message) -> () {}
    async fn channel_delete(&self, msg: Message) -> () {}
    async fn channel_pins_update(&self, msg: Message) -> () {}
    async fn thread_create(&self, msg: Message) -> () {}
    async fn thread_update(&self, msg: Message) -> () {}
    async fn thread_delete(&self, msg: Message) -> () {}
    async fn thread_list_sync(&self, msg: Message) -> () {}
    async fn thread_member_update(&self, msg: Message) -> () {}
    async fn thread_members_update(&self, msg: Message) -> () {}
    async fn entitlement_create(&self, msg: Message) -> () {}
    async fn entitlement_update(&self, msg: Message) -> () {}
    async fn entitlement_delete(&self, msg: Message) -> () {}
    async fn guild_create(&self, msg: Message) -> () {}
    async fn guild_update(&self, msg: Message) -> () {}
    async fn guild_delete(&self, msg: Message) -> () {}
    async fn guild_audit_log_entry_create(&self, msg: Message) -> () {}
    async fn guild_ban_add(&self, msg: Message) -> () {}
    async fn guild_ban_remove(&self, msg: Message) -> () {}
    async fn guild_emojis_update(&self, msg: Message) -> () {}
    async fn guild_stickers_update(&self, msg: Message) -> () {}
    async fn guild_integrations_update(&self, msg: Message) -> () {}
    async fn guild_member_add(&self, msg: Message) -> () {}
    async fn guild_member_remove(&self, msg: Message) -> () {}
    async fn guild_member_update(&self, msg: Message) -> () {}
    async fn guild_members_chunk(&self, msg: Message) -> () {}
    async fn guild_role_create(&self, msg: Message) -> () {}
    async fn guild_role_update(&self, msg: Message) -> () {}
    async fn guild_role_delete(&self, msg: Message) -> () {}
    async fn guild_scheduled_event_create(&self, msg: Message) -> () {}
    async fn guild_scheduled_event_update(&self, msg: Message) -> () {}
    async fn guild_scheduled_event_delete(&self, msg: Message) -> () {}
    async fn guild_scheduled_event_user_add(&self, msg: Message) -> () {}
    async fn guild_scheduled_event_user_remove(&self, msg: Message) -> () {}
    async fn guild_soundboard_sound_create(&self, msg: Message) -> () {}
    async fn guild_soundboard_sound_update(&self, msg: Message) -> () {}
    async fn guild_soundboard_sound_delete(&self, msg: Message) -> () {}
    async fn guild_soundboard_sounds_update(&self, msg: Message) -> () {}
    async fn soundboard_sounds(&self, msg: Message) -> () {}
    async fn integration_create(&self, msg: Message) -> () {}
    async fn integration_update(&self, msg: Message) -> () {}
    async fn integration_delete(&self, msg: Message) -> () {}
    async fn interaction_create(&self, msg: Message) -> () {}
    async fn invite_create(&self, msg: Message) -> () {}
    async fn invite_delete(&self, msg: Message) -> () {}
    async fn message_create(&self, msg: Message) -> () {}
    async fn message_update(&self, msg: Message) -> () {}
    async fn message_delete(&self, msg: Message) -> () {}
    async fn message_delete_bulk(&self, msg: Message) -> () {}
    async fn message_reaction_add(&self, msg: Message) -> () {}
    async fn message_reaction_remove(&self, msg: Message) -> () {}
    async fn message_reaction_remove_all(&self, msg: Message) -> () {}
    async fn message_reaction_remove_emoji(&self, msg: Message) -> () {}
    async fn presence_update(&self, msg: Message) -> () {}
    async fn stage_instance_create(&self, msg: Message) -> () {}
    async fn stage_instance_update(&self, msg: Message) -> () {}
    async fn stage_instance_delete(&self, msg: Message) -> () {}
    async fn subscription_create(&self, msg: Message) -> () {}
    async fn subscription_update(&self, msg: Message) -> () {}
    async fn subscription_delete(&self, msg: Message) -> () {}
    async fn typing_start(&self, msg: Message) -> () {}
    async fn user_update(&self, msg: Message) -> () {}
    async fn voice_channel_effect_send(&self, msg: Message) -> () {}
    async fn voice_state_update(&self, msg: Message) -> () {}
    async fn voice_server_update(&self, msg: Message) -> () {}
    async fn webhooks_update(&self, msg: Message) -> () {}
    async fn message_poll_vote_add(&self, msg: Message) -> () {}
    async fn message_poll_vote_remove(&self, msg: Message) -> () {}
}

// Main Discord Gateway struct
pub struct DiscordGateway {
    pub url: Option<String>,
    pub token: String,
    ws: Option<Arc<Mutex<MyWebSocket>>>,

    heartbeat_interval: Option<tokio::task::JoinHandle<()>>,
    handler: Option<Arc<dyn GatewayEventHandler>>
}

impl DiscordGateway {
    pub async fn builder(token: &str, mut url: Option<&str>) -> anyhow::Result<Self> {
        if url.is_none() {
            url = Some("wss://gateway.discord.gg/?v=10&encoding=json");
        }

        Ok(
            Self {
                token: token.to_string(),
                url: Some(url.unwrap().to_string()),
                heartbeat_interval: None,
                ws: None,
                handler: None,
            }
        )
    }

    pub async fn start(&mut self) -> anyhow::Result<()> {
        let ws = MyWebSocket::new(self.url.as_ref().unwrap()).await.map_err(|_| anyhow::anyhow!("An error has occurred while creating websocket connection"))?;

        self.ws = Some(Arc::new(Mutex::new(ws)));

        // handle event here & register identity
        if self.handler.is_none() {
            return Ok(());
        }
        let handler = self.handler.as_ref().unwrap();


        Ok(())
    }

    // Send a single heartbeat message
    async fn send_heartbeat(&mut self) -> anyhow::Result<()> {
        let ws_lock = self
            .ws
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Please build DiscordGateway first."))?;

        let mut ws = ws_lock.lock().await;
        ws.send(Message::Text("{\"op\":1,\"d\":null}".to_string().into())).await
            .map_err(|e| anyhow::anyhow!("Failed to send heartbeat: {}", e))
    }

    // Start the heartbeat loop in a separate task
    pub async fn start_heart_beat(&mut self, ms: Duration) -> anyhow::Result<bool> {
        // Check if heartbeat is already running
        if self.heartbeat_interval.is_some() {
            return Err(anyhow::anyhow!("Heartbeat is already running."));
        }

        // Send the first heartbeat
        self.send_heartbeat().await?;
        
        let ws = Arc::clone(self.ws.as_ref().ok_or_else(|| anyhow::anyhow!("Please build DiscordGateway first."))?);
        
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

    pub async fn event_handler(&mut self, handler: impl GatewayEventHandler + 'static) {
        self.handler = Some(Arc::new(handler));
    }

}