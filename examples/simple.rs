use std::env;
use wirepusher::Client;

#[tokio::main]
async fn main() -> Result<(), wirepusher::Error> {
    // Get credentials from environment variables
    let token = env::var("WIREPUSHER_TOKEN")
        .expect("WIREPUSHER_TOKEN environment variable not set");
    let device_id = env::var("WIREPUSHER_DEVICE_ID")
        .expect("WIREPUSHER_DEVICE_ID environment variable not set");

    // Create a WirePusher client
    let client = Client::new(token, device_id)?;

    // Send a simple notification
    println!("Sending simple notification...");
    let response = client
        .send("Hello from Rust!", "This is a test notification from the Rust Client Library")
        .await?;

    println!("Response status: {}", response.status);
    println!("Response message: {}", response.message);

    if response.is_success() {
        println!("✓ Notification sent successfully!");
    }

    Ok(())
}
