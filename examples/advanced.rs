use std::env;
use wirepusher::{Client, Error, Notification};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get credentials from environment variables
    let token = env::var("WIREPUSHER_TOKEN")
        .expect("WIREPUSHER_TOKEN environment variable not set");
    let user_id = env::var("WIREPUSHER_USER_ID")
        .expect("WIREPUSHER_USER_ID environment variable not set");

    // Create a WirePusher client
    let client = Client::new(token, user_id)?;

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
        }
        Err(Error::Authentication { message, status_code }) => {
            eprintln!("✗ Authentication error (HTTP {}):", status_code);
            eprintln!("  {}", message);
            eprintln!("  Check your WIREPUSHER_TOKEN and WIREPUSHER_USER_ID");
        }
        Err(Error::Validation { message, status_code }) => {
            eprintln!("✗ Validation error (HTTP {}):", status_code);
            eprintln!("  {}", message);
        }
        Err(Error::RateLimit { message, status_code }) => {
            eprintln!("✗ Rate limit exceeded (HTTP {}):", status_code);
            eprintln!("  {}", message);
        }
        Err(e) => {
            eprintln!("✗ Error: {}", e);
        }
    }

    Ok(())
}
