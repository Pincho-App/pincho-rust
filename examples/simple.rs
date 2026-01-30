use pincho::Client;

#[tokio::main]
async fn main() -> Result<(), pincho::Error> {
    // Create a Pincho client from environment
    // Reads PINCHO_TOKEN from environment
    let client = Client::from_env()?;

    // Or provide token explicitly:
    // let client = Client::new("your_token")?;

    // Send a simple notification
    println!("Sending simple notification...");
    let response = client
        .send(
            "Hello from Rust!",
            "This is a test notification from the Rust Client Library",
        )
        .await?;

    println!("Response status: {}", response.status);
    println!("Response message: {}", response.message);

    if response.is_success() {
        println!("✓ Notification sent successfully!");
    }

    // Check rate limit info
    if let Some(rate_limit) = client.get_last_rate_limit() {
        println!("\nRate limit info:");
        println!("  Remaining: {}/{}", rate_limit.remaining, rate_limit.limit);
        println!("  Resets at: {}", rate_limit.reset);
    }

    Ok(())
}
