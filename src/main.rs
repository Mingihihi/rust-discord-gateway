use dotenv;
use std::time::Duration;

mod ws;

mod discord_gateway;
use discord_gateway::*;

#[tokio::main]
async fn main() {
    // Load token from dotenv
    dotenv::dotenv().ok();
    let token = std::env::var("TOKEN").expect("Cannot find token from .env file.");
    // Create a new Discord Gateway instance
    let mut gateway = DiscordGateway::connect(&token, None).await.expect("Failed to connect to Discord Gateway");
    
    println!("Connected to Discord Gateway");
    
    // Start the heartbeat with a 45-second interval
    match gateway.start_heart_beat(Duration::from_secs(45)).await {
        Ok(_) => println!("Heartbeat started successfully"),
        Err(e) => eprintln!("Failed to start heartbeat: {}", e),
    }
    
    // // Read the first message (usually the Hello message)
    // if let Some(Ok(msg)) = gateway.read.next().await {
    //     println!("Received:\n{:?}", serde_json::from_str::<serde_json::Value>(msg.to_string().as_str()).unwrap());
    // } else {
    //     println!("No message received");
    // }
    
    // Wait for a while to see heartbeats in action
    tokio::time::sleep(Duration::from_secs(90)).await;
    
    // Stop the heartbeat
    match gateway.stop_heart_beat().await {
        Ok(_) => println!("Heartbeat stopped successfully"),
        Err(e) => eprintln!("Failed to stop heartbeat: {}", e),
    }
    
    println!("Disconnected from Discord Gateway");
}