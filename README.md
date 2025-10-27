# WirePusher Rust SDK

[![Crates.io](https://img.shields.io/crates/v/wirepusher.svg)](https://crates.io/crates/wirepusher)
[![Documentation](https://docs.rs/wirepusher/badge.svg)](https://docs.rs/wirepusher)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Official Rust SDK for [WirePusher](https://wirepusher.dev) - Send push notifications to your mobile devices.

## Features

- **Async/await support** - Built on tokio for high-performance async operations
- **Type-safe builder pattern** - Compile-time guarantees for notification construction
- **Connection pooling** - Automatic HTTP connection reuse via reqwest
- **Comprehensive error handling** - Detailed error types with thiserror
- **Zero-copy where possible** - Efficient memory usage
- **Thread-safe** - All types are `Send + Sync`
- **>90% test coverage** - Thoroughly tested with unit and integration tests

## Requirements

- Rust 2021 edition or later
- Tokio runtime for async operations

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
wirepusher = "1.0"
tokio = { version = "1", features = ["full"] }
```

Or install via cargo:

```bash
cargo add wirepusher
cargo add tokio --features full
```

## Quick Start

```rust
use wirepusher::Client;

#[tokio::main]
async fn main() -> Result<(), wirepusher::Error> {
    let client = Client::new("wpt_your_token", "your_user_id")?;

    // Send a simple notification
    let response = client.send("Build Complete", "v1.2.3 deployed successfully").await?;

    if response.is_success() {
        println!("Notification sent successfully!");
    }

    Ok(())
}
```

## Usage Examples

### Simple Notification

```rust
use wirepusher::Client;

#[tokio::main]
async fn main() -> Result<(), wirepusher::Error> {
    let client = Client::new("wpt_your_token", "your_user_id")?;

    let response = client.send("Hello", "World!").await?;

    if response.is_success() {
        println!("Notification sent successfully!");
    }

    Ok(())
}
```

### Advanced Notification with Builder

```rust
use wirepusher::{Client, Notification};

#[tokio::main]
async fn main() -> Result<(), wirepusher::Error> {
    let client = Client::new("wpt_your_token", "your_user_id")?;

    let notification = Notification::builder()
        .title("Deploy Complete")
        .message("Version 1.2.3 deployed to production")
        .notification_type("deployment")
        .tags(vec!["production".to_string(), "release".to_string()])
        .image_url("https://example.com/success.png")
        .action_url("https://example.com/deployment/123")
        .build()?;

    let response = client.send_notification(notification).await?;
    println!("Status: {}", response.status);

    Ok(())
}
```

### Error Handling

```rust
use wirepusher::{Client, Error};

#[tokio::main]
async fn main() {
    let client = Client::new("wpt_token", "user_id").unwrap();

    match client.send("Test", "Message").await {
        Ok(response) => println!("Success: {}", response.message),
        Err(Error::Authentication { message, status_code }) => {
            eprintln!("Auth error ({}): {}", status_code, message);
        }
        Err(Error::Validation { message, status_code }) => {
            eprintln!("Validation error ({}): {}", status_code, message);
        }
        Err(Error::RateLimit { message, status_code }) => {
            eprintln!("Rate limit ({}): {}", status_code, message);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

### Environment Variables

Store credentials in environment variables:

```bash
export WIREPUSHER_TOKEN="wpt_your_token_here"
export WIREPUSHER_USER_ID="your_user_id"
```

```rust
use std::env;
use wirepusher::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = env::var("WIREPUSHER_TOKEN")?;
    let user_id = env::var("WIREPUSHER_USER_ID")?;

    let client = Client::new(token, user_id)?;
    client.send("Test", "Message").await?;

    Ok(())
}
```

### Concurrent Requests

The client is thread-safe and can be cloned for concurrent use:

```rust
use wirepusher::Client;

#[tokio::main]
async fn main() -> Result<(), wirepusher::Error> {
    let client = Client::new("wpt_token", "user_id")?;

    // Send multiple concurrent notifications
    let futures: Vec<_> = (0..10)
        .map(|i| {
            let client = client.clone();
            async move {
                client.send(format!("Notification {}", i), "Message").await
            }
        })
        .collect();

    let results = futures::future::join_all(futures).await;

    for (i, result) in results.into_iter().enumerate() {
        match result {
            Ok(_) => println!("Notification {} sent", i),
            Err(e) => eprintln!("Notification {} failed: {}", i, e),
        }
    }

    Ok(())
}
```

## API Reference

### `Client`

Main client for sending notifications.

#### Constructors

```rust
// Default client
Client::new(token: impl Into<String>, user_id: impl Into<String>) -> Result<Self>

// Custom base URL (for testing)
Client::with_base_url(
    token: impl Into<String>,
    user_id: impl Into<String>,
    base_url: impl Into<String>
) -> Result<Self>
```

**Parameters:**
- `token` (required): Your WirePusher API token (starts with `wpt_`)
- `user_id` (required): Your WirePusher user ID
- `base_url` (optional): Custom API base URL

#### Methods

```rust
// Simple send
async fn send(
    &self,
    title: impl Into<String>,
    message: impl Into<String>
) -> Result<NotificationResponse>

// Advanced send
async fn send_notification(
    &self,
    notification: Notification
) -> Result<NotificationResponse>
```

### `Notification`

Represents a notification to be sent.

**Properties:**
- `title` (required): Notification title (max 256 characters)
- `message` (required): Notification message (max 4096 characters)
- `notification_type` (optional): Notification type for categorization
- `tags` (optional): Array of tags for filtering (max 10)
- `image_url` (optional): URL to an image to display
- `action_url` (optional): URL to open when notification is tapped

**Construction:**

```rust
// Simple
Notification::new(title, message)

// Builder
Notification::builder()
    .title("Title")
    .message("Message")
    .notification_type("info")
    .tags(vec!["tag1".to_string(), "tag2".to_string()])
    .image_url("https://example.com/image.png")
    .action_url("https://example.com")
    .build()?
```

### `NotificationResponse`

Response from sending a notification.

**Properties:**
- `status`: Response status (e.g., "success")
- `message`: Response message

**Methods:**
- `is_success()`: Returns true if status is "success"

### Error Types

All errors implement the `Error` trait via thiserror.

#### `Error` Variants

- **`Authentication { message, status_code }`** - Authentication failures (401, 403)
- **`Validation { message, status_code }`** - Invalid parameters (400, 404)
- **`RateLimit { message, status_code }`** - Rate limit exceeded (429)
- **`Request(reqwest::Error)`** - HTTP request errors
- **`Json(serde_json::Error)`** - JSON serialization errors
- **`Api { message, status_code }`** - Other API errors
- **`InvalidConfig(String)`** - Invalid client configuration
- **`BuilderValidation(String)`** - Notification builder validation errors

#### Error Methods

```rust
fn status_code(&self) -> Option<u16>
fn is_authentication_error(&self) -> bool
fn is_validation_error(&self) -> bool
fn is_rate_limit_error(&self) -> bool
```

## Best Practices

### Client Reuse

Create a single client instance and reuse it for better performance:

```rust
// Good: Reuse client
let client = Client::new("token", "user_id")?;
for i in 0..10 {
    client.send(format!("Notification {}", i), "Message").await?;
}

// Bad: Create new client each time (slower)
for i in 0..10 {
    let client = Client::new("token", "user_id")?;
    client.send(format!("Notification {}", i), "Message").await?;
}
```

### Graceful Degradation

Don't let notification failures break your application:

```rust
async fn send_notification_safe(client: &Client, title: &str, message: &str) {
    match client.send(title, message).await {
        Ok(_) => tracing::info!("Notification sent successfully"),
        Err(e) => tracing::error!("Failed to send notification: {}", e),
    }
    // Continue execution regardless of result
}
```

### Environment-Based Configuration

Use different configurations for dev/staging/prod:

```rust
use std::env;

fn get_client() -> Result<Client, wirepusher::Error> {
    let token = env::var("WIREPUSHER_TOKEN")
        .expect("WIREPUSHER_TOKEN must be set");
    let user_id = env::var("WIREPUSHER_USER_ID")
        .expect("WIREPUSHER_USER_ID must be set");

    Client::new(token, user_id)
}
```

## Development

### Building

```bash
cargo build
```

### Running Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_send_simple_notification_success
```

### Running Examples

```bash
# Set credentials
export WIREPUSHER_TOKEN="wpt_your_token"
export WIREPUSHER_USER_ID="your_user_id"

# Run simple example
cargo run --example simple

# Run advanced example
cargo run --example advanced
```

### Code Coverage

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html --output-dir coverage
```

### Generating Documentation

```bash
cargo doc --open
```

## Project Structure

```
rust-sdk/
├── Cargo.toml
├── src/
│   ├── lib.rs          # Public API exports
│   ├── client.rs       # Client implementation
│   ├── notification.rs # Notification + Builder
│   ├── response.rs     # NotificationResponse
│   └── error.rs        # Error types
├── tests/
│   └── integration_tests.rs
├── examples/
│   ├── simple.rs
│   └── advanced.rs
└── README.md
```

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for version history.

## Security

For security issues, please see [SECURITY.md](SECURITY.md).

## License

MIT License - see [LICENSE](LICENSE) for details.

## Links

- [WirePusher Website](https://wirepusher.dev)
- [API Documentation](https://wirepusher.dev/api)
- [Rust Documentation](https://docs.rs/wirepusher)
- [Crates.io](https://crates.io/crates/wirepusher)
- [GitLab Repository](https://gitlab.com/wirepusher/rust-sdk)
- [Issue Tracker](https://gitlab.com/wirepusher/rust-sdk/-/issues)

## Support

- Email: support@wirepusher.dev
- Issues: https://gitlab.com/wirepusher/rust-sdk/-/issues
