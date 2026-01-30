use pincho::{Client, Error, Notification};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a Pincho client from environment
    let client = Client::from_env()?;

    // Build a notification with all options
    println!("Building advanced notification...");
    let notification = Notification::builder()
        .title("Deploy Complete")
        .message("Version 1.2.3 has been deployed to production")
        .notification_type("deployment")
        .tags(vec![
            "production".to_string(),
            "release".to_string(),
            "v1.2.3".to_string(),
        ])
        .image_url("https://via.placeholder.com/300x200?text=Success")
        .action_url("https://example.com/deployments/123")
        .build()?;

    println!("\nNotification details:");
    println!("  Title: {}", notification.title);
    println!("  Message: {}", notification.message);
    println!(
        "  Type: {}",
        notification.notification_type.as_ref().unwrap()
    );
    println!("  Tags: {:?}", notification.tags.as_ref().unwrap());
    println!("  Image URL: {:?}", notification.image_url);
    println!("  Action URL: {:?}", notification.action_url);

    // Send the notification with error handling
    println!("\nSending notification...");
    match client.send_notification(notification).await {
        Ok(response) => {
            println!("✓ Success!");
            println!("  Status: {}", response.status);
            println!("  Message: {}", response.message);

            // Check rate limit info
            if let Some(rate_limit) = client.get_last_rate_limit() {
                println!("\nRate limit info:");
                println!("  Remaining: {}/{}", rate_limit.remaining, rate_limit.limit);
            }
        }
        Err(Error::Authentication {
            message,
            status_code,
        }) => {
            eprintln!("✗ Authentication error (HTTP {}):", status_code);
            eprintln!("  {}", message);
            eprintln!("  Check your PINCHO_TOKEN");
        }
        Err(Error::Validation {
            message,
            status_code,
        }) => {
            eprintln!("✗ Validation error (HTTP {}):", status_code);
            eprintln!("  {}", message);
        }
        Err(Error::RateLimit {
            message,
            status_code,
        }) => {
            eprintln!("✗ Rate limit exceeded (HTTP {}):", status_code);
            eprintln!("  {}", message);
        }
        Err(e) => {
            eprintln!("✗ Error: {}", e);
            if e.is_retryable() {
                eprintln!("  This error is retryable");
            }
        }
    }

    // Example: NotifAI (AI-powered notification generation)
    println!("\n--- NotifAI Example ---");
    match client
        .notifai("deployment finished v2.1.3 is live", None)
        .await
    {
        Ok(response) => {
            println!("✓ NotifAI generated notification:");
            println!("  Title: {}", response.notification.title);
            println!("  Message: {}", response.notification.message);
        }
        Err(e) => {
            eprintln!("✗ NotifAI error: {}", e);
        }
    }

    Ok(())
}
