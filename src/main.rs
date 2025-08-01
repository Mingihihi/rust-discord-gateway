use dotenv;

mod ws;

mod discord_gateway;
use discord_gateway::*;

#[tokio::main]
async fn main() {
    // Load token from dotenv
    dotenv::dotenv().ok();
    let token = std::env::var("TOKEN").expect("Cannot find token from .env file.");

    // Create a new Discord Gateway instance
    let mut gateway = DiscordGateway::builder(&token, None).await.expect("Failed to connect to Discord Gateway");
    
    println!("Connected to Discord Gateway");
    
    // Heart beat will be automatically started when starting gateway
    // // Start the heartbeat with a 45-second interval
    // match gateway.start_heart_beat(Duration::from_secs(45)).await {
    //     Ok(_) => println!("Heartbeat started successfully"),
    //     Err(e) => eprintln!("Failed to start heartbeat: {}", e),
    // }
    
    // // Wait for a while to see heartbeats in action
    // tokio::time::sleep(Duration::from_secs(90)).await;
    
    // // Stop the heartbeat
    // match gateway.stop_heart_beat().await {
    //     Ok(_) => println!("Heartbeat stopped successfully"),
    //     Err(e) => eprintln!("Failed to stop heartbeat: {}", e),
    // }

    gateway.start().await.expect("An error has occurred while starting gateway");
    
    println!("Disconnected from Discord Gateway");
}