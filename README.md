# WirePusher Rust Client Library

[![crates.io](https://img.shields.io/crates/v/wirepusher.svg)](https://crates.io/crates/wirepusher)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Official Rust client for [WirePusher](https://wirepusher.dev) push notifications.

## Installation

```toml
[dependencies]
wirepusher = "1.0.0-alpha.1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use wirepusher::Client;

#[tokio::main]
async fn main() -> Result<(), wirepusher::Error> {
    // Auto-load token from WIREPUSHER_TOKEN env var
    let client = Client::from_env()?;
    client.send("Deploy Complete", "Version 1.2.3 deployed").await?;

    // Or provide token explicitly
    let client = Client::new("YOUR_TOKEN")?;
    client.send("Alert", "Server CPU at 95%").await?;
    Ok(())
}
```

## Features

```rust
use wirepusher::{Client, Notification};

#[tokio::main]
async fn main() -> Result<(), wirepusher::Error> {
    let client = Client::from_env()?;

    // Full parameters
    let notification = Notification::builder()
        .title("Deploy Complete")
        .message("Version 1.2.3 deployed")
        .notification_type("deployment")
        .tags(vec!["production".to_string(), "backend".to_string()])
        .image_url("https://example.com/success.png")
        .action_url("https://example.com/deploy/123")
        .build()?;

    client.send_notification(notification).await?;

    // AI-powered notifications (NotifAI)
    let response = client.notifai("deployment finished, v2.1.3 is live", None).await?;
    println!("Title: {}", response.notification.title);   // AI-generated
    println!("Message: {}", response.notification.message);

    // Encrypted messages
    let notification = Notification::builder()
        .title("Security Alert")
        .message("Sensitive data")
        .notification_type("security")
        .encryption_password("your_password")
        .build()?;
    client.send_notification(notification).await?;

    Ok(())
}
```

## Configuration

```rust
// Environment variables (recommended)
// WIREPUSHER_TOKEN - API token (required if not passed to constructor)
// WIREPUSHER_TIMEOUT - Request timeout in seconds (default: 30)
// WIREPUSHER_MAX_RETRIES - Retry attempts (default: 3)

// Or explicit configuration
let client = Client::with_config("YOUR_TOKEN", 60, 5)?;
```

## Error Handling

```rust
use wirepusher::{Client, Error};

#[tokio::main]
async fn main() {
    let client = Client::from_env().unwrap();

    match client.send("Title", "Message").await {
        Ok(response) => println!("Success: {}", response.message),
        Err(Error::Authentication { message, .. }) => {
            eprintln!("Invalid token: {}", message);
        }
        Err(Error::Validation { message, .. }) => {
            eprintln!("Invalid parameters: {}", message);
        }
        Err(Error::RateLimit { message, .. }) => {
            eprintln!("Rate limited: {}", message);
        }
        Err(e) => {
            if e.is_retryable() {
                eprintln!("Retryable error: {}", e);
            }
        }
    }
}
```

Automatic retry with exponential backoff for network errors, 5xx, and 429 (rate limit).

## Smart Rate Limiting

```rust
let client = Client::from_env()?;
client.send("Alert", "Message").await?;

// Check rate limit status after any request
if let Some(rate_limit) = client.get_last_rate_limit() {
    println!("Remaining: {}/{}", rate_limit.remaining, rate_limit.limit);
    println!("Resets at: {}", rate_limit.reset);
}
```

## Links

- **Get Token**: App → Settings → Help → copy token
- **Documentation**: https://wirepusher.dev/help
- **Repository**: https://gitlab.com/wirepusher/wirepusher-rust
- **Crates.io**: https://crates.io/crates/wirepusher

## License

MIT
