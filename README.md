# WirePusher Rust Client Library

[![Crates.io](https://img.shields.io/crates/v/wirepusher.svg)](https://crates.io/crates/wirepusher)
[![Documentation](https://docs.rs/wirepusher/badge.svg)](https://docs.rs/wirepusher)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Official Rust Client Library for [WirePusher](https://wirepusher.dev) push notifications.

## Features

- ✅ **Async/Await** - Built on tokio for high-performance operations
- ✅ **Type-Safe Builder** - Compile-time guarantees for notification construction
- ✅ **Connection Pooling** - Automatic HTTP connection reuse via reqwest
- ✅ **Thread-Safe** - All types are `Send + Sync`
- ✅ **Well-Tested** - >90% test coverage

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
wirepusher = "1.0.0"
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
    let client = Client::new("abc12345", "YOUR_DEVICE_ID")?;

    client.send(
        "Deploy Complete",
        "Version 1.2.3 deployed to production"
    ).await?;

    Ok(())
}
```

**Get your credentials:** Open app → Settings → Help → copy token and device ID

## Usage

### Basic Example

```rust
use wirepusher::Client;

#[tokio::main]
async fn main() -> Result<(), wirepusher::Error> {
    let client = Client::new("abc12345", "your_device_id")?;

    let response = client.send(
        "Deploy Complete",
        "Version 1.2.3 deployed to production"
    ).await?;

    println!("Status: {}", response.status);  // "success"
    Ok(())
}
```

### All Parameters

```rust
use wirepusher::{Client, Notification};

#[tokio::main]
async fn main() -> Result<(), wirepusher::Error> {
    let client = Client::new("abc12345", "your_device_id")?;

    let notification = Notification::builder()
        .title("Deploy Complete")
        .message("Version 1.2.3 deployed to production")
        .notification_type("deployment")
        .tags(vec!["production".to_string(), "backend".to_string()])
        .image_url("https://cdn.example.com/success.png")
        .action_url("https://dash.example.com/deploy/123")
        .build()?;

    client.send_notification(notification).await?;
    Ok(())
}
```

### Environment Variables

```rust
use std::env;
use wirepusher::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = env::var("WIREPUSHER_TOKEN")?;
    let device_id = env::var("WIREPUSHER_DEVICE_ID")?;
    let client = Client::new(token, device_id)?;

    client.send(
        "Deploy Complete",
        "Version 1.2.3 deployed to production"
    ).await?;

    Ok(())
}
```

## Encryption

Encrypt notification messages using AES-128-CBC. Only the `message` field is encrypted—`title`, `type`, `tags`, `image_url`, and `action_url` remain unencrypted for filtering and display.

**Setup:**
1. In the app, create a notification type
2. Set an encryption password for that type
3. Pass the same `type` and password when sending

```rust
use std::env;
use wirepusher::{Client, Notification};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = env::var("WIREPUSHER_TOKEN")?;
    let device_id = env::var("WIREPUSHER_DEVICE_ID")?;
    let password = env::var("WIREPUSHER_ENCRYPTION_PASSWORD")?;

    let client = Client::new(token, device_id)?;

    let notification = Notification::builder()
        .title("Security Alert")
        .message("Unauthorized access attempt detected")
        .notification_type("security")
        .encryption_password(password)
        .build()?;

    client.send_notification(notification).await?;
    Ok(())
}
```

**Security notes:**
- Use strong passwords (minimum 12 characters)
- Store passwords securely (environment variables, secret managers)
- Password must match the type configuration in the app

## API Reference

### Client

**Constructor:**
```rust
Client::new(token: impl Into<String>, device_id: impl Into<String>) -> Result<Self>
```

**Parameters:**
- `token` (required): Your WirePusher token
- `device_id` (required): Your device ID for identification

**Methods:**

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

### Notification

Build notifications using the builder pattern.

**Properties:**
- `title` (str, required): Notification title
- `message` (str, required): Notification message
- `notification_type` (str, optional): Category for organization
- `tags` (list[str], optional): Tags for filtering
- `image_url` (str, optional): Image URL to display
- `action_url` (str, optional): URL to open when tapped
- `encryption_password` (str, optional): Password for encryption

**Construction:**

```rust
// Builder pattern
Notification::builder()
    .title("Deploy Complete")
    .message("Version 1.2.3 deployed to production")
    .notification_type("deployment")
    .tags(vec!["production".to_string(), "backend".to_string()])
    .image_url("https://cdn.example.com/success.png")
    .action_url("https://dash.example.com/deploy/123")
    .build()?
```

### NotificationResponse

**Properties:**
- `status`: Response status (e.g., "success")
- `message`: Response message

**Methods:**
- `is_success()`: Returns true if status is "success"

### Error Types

All errors implement the `Error` trait via thiserror.

**Variants:**
- `Authentication { message, status_code }`: Invalid token (401, 403)
- `Validation { message, status_code }`: Invalid parameters (400, 404)
- `RateLimit { message, status_code }`: Rate limit exceeded (429)
- `Request(reqwest::Error)`: HTTP request errors
- `Json(serde_json::Error)`: JSON serialization errors
- `Api { message, status_code }`: Other API errors
- `InvalidConfig(String)`: Invalid client configuration
- `BuilderValidation(String)`: Notification builder validation errors

**Methods:**
```rust
fn status_code(&self) -> Option<u16>
fn is_authentication_error(&self) -> bool
fn is_validation_error(&self) -> bool
fn is_rate_limit_error(&self) -> bool
```

## Error Handling

```rust
use wirepusher::{Client, Error};

#[tokio::main]
async fn main() {
    let client = Client::new("abc12345", "device_id").unwrap();

    match client.send("Test", "Message").await {
        Ok(response) => println!("Success: {}", response.message),
        Err(Error::Authentication { message, status_code }) => {
            eprintln!("Authentication failed ({}): {}", status_code, message);
        }
        Err(Error::Validation { message, status_code }) => {
            eprintln!("Invalid parameters ({}): {}", status_code, message);
        }
        Err(Error::RateLimit { message, status_code }) => {
            eprintln!("Rate limit exceeded ({}): {}", status_code, message);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

## Examples

### CI/CD Pipeline

```rust
use std::env;
use wirepusher::Client;

async fn notify_deployment(version: &str, environment: &str) -> Result<(), Box<dyn std::error::Error>> {
    let token = env::var("WIREPUSHER_TOKEN")?;
    let device_id = env::var("WIREPUSHER_DEVICE_ID")?;
    let client = Client::new(token, device_id)?;

    client.send(
        "Deploy Complete",
        format!("Version {} deployed to {}", version, environment)
    ).await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    notify_deployment("1.2.3", "production").await.unwrap();
}
```

### Server Monitoring

```rust
use std::env;
use wirepusher::{Client, Notification};

async fn check_server_health() -> Result<(), Box<dyn std::error::Error>> {
    let cpu_usage = get_cpu_usage(); // Your CPU monitoring logic
    let memory_usage = get_memory_usage(); // Your memory monitoring logic

    if cpu_usage > 80.0 || memory_usage > 80.0 {
        let token = env::var("WIREPUSHER_TOKEN")?;
        let device_id = env::var("WIREPUSHER_DEVICE_ID")?;
        let client = Client::new(token, device_id)?;

        let notification = Notification::builder()
            .title("Server Alert")
            .message(format!("CPU: {:.1}%, Memory: {:.1}%", cpu_usage, memory_usage))
            .notification_type("alert")
            .tags(vec!["server".to_string(), "critical".to_string()])
            .build()?;

        client.send_notification(notification).await?;
    }

    Ok(())
}
```

### Concurrent Requests

The client is thread-safe and can be cloned for concurrent use:

```rust
use wirepusher::Client;

#[tokio::main]
async fn main() -> Result<(), wirepusher::Error> {
    let client = Client::new("abc12345", "device_id")?;

    // Send multiple concurrent notifications
    let futures: Vec<_> = (0..10)
        .map(|i| {
            let client = client.clone();
            async move {
                client.send(
                    format!("Notification {}", i),
                    "Message"
                ).await
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

## Troubleshooting

### Common Issues

#### Authentication Errors (401/403)

**Error:** `Authentication { message: "Invalid token", status_code: 401 }`

**Solutions:**
- Verify your token is correct
- Check that your token hasn't expired or been revoked
- Ensure you're using the correct token from Settings → Help in the app
- Verify the device ID matches your current device

#### Validation Errors (400/404)

**Error:** `Validation { message: "Invalid parameters", status_code: 400 }`

**Solutions:**
- Ensure title is not empty and ≤256 characters
- Ensure message is not empty and ≤4096 characters
- Verify URLs (image_url, action_url) are valid and well-formed
- Check tags: max 10 tags, each ≤50 characters, no empty tags
- Confirm notification type matches configured types (if using encryption)

#### Rate Limit Errors (429)

**Error:** `RateLimit { message: "Rate limit exceeded", status_code: 429 }`

**Solutions:**
- Implement exponential backoff between requests
- Reduce notification frequency
- Contact support for rate limit increase if needed

#### Connection/Timeout Errors

**Error:** `Request(reqwest::Error { kind: Timeout, ... })`

**Solutions:**
- Check your internet connection
- Verify firewall isn't blocking HTTPS to wirepusher.com
- Increase timeout if needed (default is 30 seconds)
- Check WirePusher service status at https://status.wirepusher.dev

#### Encryption Issues

**Symptom:** Message appears garbled or empty in notification

**Solutions:**
- Ensure the notification type in the app has encryption enabled
- Verify the password matches exactly (case-sensitive)
- Confirm the type parameter matches the configured type name
- Check that encryption_password is set before calling build()

#### URL Validation Errors

**Error:** `BuilderValidation("invalid image_url: ...")`

**Solutions:**
- Ensure URLs start with `http://` or `https://`
- Check for typos in the URL
- Verify the URL is properly encoded
- Use absolute URLs, not relative paths

#### Tag Validation Errors

**Error:** `BuilderValidation("tag at index 0 exceeds 50 characters")`

**Solutions:**
- Keep individual tags under 50 characters
- Use abbreviations or shorter identifiers
- Split long tags into multiple shorter tags
- Remove empty tags from the array

### Debugging Tips

1. **Enable verbose logging:**
   ```rust
   env_logger::init(); // Add this to see detailed HTTP logs
   ```

2. **Check response details:**
   ```rust
   match client.send("Title", "Message").await {
       Ok(response) => println!("Response: {:?}", response),
       Err(e) => eprintln!("Error: {:?}", e),
   }
   ```

3. **Validate notifications before sending:**
   ```rust
   let notification = Notification::builder()
       .title("Test")
       .message("Message")
       .build()?; // Catches validation errors early
   ```

4. **Test with minimal example:**
   ```rust
   // Simplest possible test
   let client = Client::new("abc12345", "device_id")?;
   client.send("Test", "Message").await?;
   ```

### Getting Help

- **Documentation:** https://docs.rs/wirepusher
- **Issues:** https://gitlab.com/wirepusher/wirepusher-rust/-/issues
- **Support:** support@wirepusher.dev
- **Status:** https://status.wirepusher.dev

## Development

### Setup

```bash
# Clone repository
git clone https://gitlab.com/wirepusher/wirepusher-rust.git
cd wirepusher-rust

# Build
cargo build
```

### Testing

```bash
# Run tests
cargo test

# Run with output
cargo test -- --nocapture

# Generate coverage report
cargo install cargo-tarpaulin
cargo tarpaulin --out Html --output-dir coverage
```

### Running Examples

```bash
# Set credentials
export WIREPUSHER_TOKEN="abc12345"
export WIREPUSHER_DEVICE_ID="YOUR_DEVICE_ID"

# Run examples
cargo run --example simple
cargo run --example advanced
```

### Documentation

```bash
cargo doc --open
```

## Requirements

- Rust 2021 edition or later
- Tokio runtime for async operations

## Links

- **Documentation**: https://docs.rs/wirepusher
- **Repository**: https://gitlab.com/wirepusher/wirepusher-rust
- **Issues**: https://gitlab.com/wirepusher/wirepusher-rust/-/issues
- **Crates.io**: https://crates.io/crates/wirepusher

## License

MIT License - see [LICENSE](LICENSE) file for details.
