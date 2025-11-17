# Advanced Usage

## Rate Limit Monitoring

The client automatically parses rate limit headers after each request:

```rust
use wirepusher::Client;

#[tokio::main]
async fn main() -> Result<(), wirepusher::Error> {
    let client = Client::from_env()?;
    client.send("Test", "Message").await?;

    if let Some(rate_limit) = client.get_last_rate_limit() {
        println!("Limit: {}", rate_limit.limit);
        println!("Remaining: {}", rate_limit.remaining);
        println!("Reset: {}", rate_limit.reset);

        if rate_limit.remaining < 10 {
            println!("Warning: Low on rate limit quota");
        }
    }
    Ok(())
}
```

Rate limit information is thread-safe and shared across clones of the client.

## Automatic Retry Logic

The client automatically retries failed requests with exponential backoff:

- **Retryable errors**: Rate limits (429), server errors (5xx), network errors
- **Non-retryable**: Authentication (401/403), validation (400/404)
- **Backoff**: 1s, 2s, 4s, 8s... capped at 30 seconds
- **Default retries**: 3 (configurable via `WIREPUSHER_MAX_RETRIES` or `with_config`)

```rust
// Configure retries
let client = Client::with_config("token", 30, 5)?; // 5 retries

// Check if an error is retryable
match client.send("Test", "Message").await {
    Err(e) if e.is_retryable() => {
        println!("Retryable error: {}", e);
    }
    Err(e) => println!("Non-retryable error: {}", e),
    Ok(_) => println!("Success"),
}
```

## Encryption

Messages can be encrypted using AES-128-CBC. Only the message field is encrypted:

```rust
use wirepusher::{Client, Notification};

#[tokio::main]
async fn main() -> Result<(), wirepusher::Error> {
    let client = Client::from_env()?;

    let notification = Notification::builder()
        .title("Security Alert")           // NOT encrypted
        .message("Sensitive data here")    // ENCRYPTED
        .notification_type("security")     // NOT encrypted
        .encryption_password("your_password")
        .build()?;

    client.send_notification(notification).await?;
    Ok(())
}
```

**Requirements**:
- Configure the notification type in the app with the same password
- Password must match exactly (case-sensitive)
- Uses SHA1-based key derivation (for app compatibility)
- IV sent as hex in the `iv` JSON field

## Custom HTTP Client

The client uses reqwest with connection pooling and HTTP/2 support by default:

```rust
use wirepusher::Client;

// Custom timeout
let client = Client::with_config("token", 60, 3)?; // 60 second timeout

// Custom base URL (for testing)
let mut client = Client::new("token")?;
client.set_base_url("https://custom.api.com");
```

## Tag Normalization

Tags are automatically normalized:
- Trimmed of whitespace
- Converted to lowercase
- Invalid characters removed (only alphanumeric, `-`, `_` allowed)
- Empty tags filtered out
- Duplicates removed

```rust
let notification = Notification::builder()
    .title("Test")
    .message("Message")
    .tags(vec![
        "  Production  ".into(),  // becomes "production"
        "BACKEND".into(),          // becomes "backend"
        "test-tag_123".into(),     // stays "test-tag_123"
        "special@chars!".into(),   // becomes "specialchars"
    ])
    .build()?;
```

## Concurrent Usage

The client is `Send + Sync` and can be cloned for concurrent use:

```rust
use wirepusher::Client;
use futures::future::join_all;

#[tokio::main]
async fn main() -> Result<(), wirepusher::Error> {
    let client = Client::from_env()?;

    let futures: Vec<_> = (0..10)
        .map(|i| {
            let client = client.clone();
            async move { client.send(format!("Task {}", i), "Complete").await }
        })
        .collect();

    let results = join_all(futures).await;
    for (i, result) in results.iter().enumerate() {
        match result {
            Ok(_) => println!("Task {} sent", i),
            Err(e) => eprintln!("Task {} failed: {}", i, e),
        }
    }
    Ok(())
}
```

## Error Handling Best Practices

```rust
use wirepusher::{Client, Error};

async fn send_with_handling(client: &Client) {
    match client.send("Test", "Message").await {
        Ok(response) => {
            println!("Success: {}", response.message);

            // Check remaining quota
            if let Some(rl) = client.get_last_rate_limit() {
                if rl.remaining < 5 {
                    println!("Warning: Low rate limit quota");
                }
            }
        }
        Err(Error::Authentication { message, status_code }) => {
            eprintln!("Auth error ({}): {}", status_code, message);
            // Token is invalid, need to refresh
        }
        Err(Error::Validation { message, status_code }) => {
            eprintln!("Validation error ({}): {}", status_code, message);
            // Fix request parameters
        }
        Err(Error::RateLimit { message, status_code }) => {
            eprintln!("Rate limit ({}): {}", status_code, message);
            // Already retried, wait longer
        }
        Err(e) => {
            if e.is_retryable() {
                eprintln!("Retryable error (retries exhausted): {}", e);
            } else {
                eprintln!("Error: {}", e);
            }
        }
    }
}
```
