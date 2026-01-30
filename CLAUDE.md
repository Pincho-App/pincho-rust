# CLAUDE.md - Pincho Rust Client Library

## Overview

Rust client for Pincho push notifications. Rust 2021+, reqwest + tokio, async/await.

## Structure

```
src/
├── lib.rs              # Public API exports
├── client.rs           # Client implementation (Bearer auth, retry, rate limits)
├── notification.rs     # Notification builder pattern
├── response.rs         # Response structs (NotificationResponse, NotifAIResponse, RateLimitInfo)
├── error.rs            # Error types (thiserror)
└── encryption.rs       # AES-128-CBC encryption
```

## API

```rust
let client = Client::from_env()?;           // From PINCHO_TOKEN
let client = Client::new("TOKEN")?;         // Explicit token
let client = Client::with_config("TOKEN", timeout, max_retries)?;

// Send notifications
client.send("Title", "Message").await?;
client.send_notification(notification).await?;

// NotifAI (AI-powered)
client.notifai("text", None).await?;
client.notifai("text", Some("type".into())).await?;

// Rate limits
client.get_last_rate_limit();  // Returns Option<RateLimitInfo>
```

## Exceptions

- `Authentication { message, status_code }` (401/403) - not retryable
- `Validation { message, status_code }` (400/404) - not retryable
- `RateLimit { message, status_code }` (429) - retryable
- `Api { message, status_code }` (5xx) - retryable
- `Request(reqwest::Error)` - retryable for network errors

All have `is_retryable()` method.

## Config

Env vars: `PINCHO_TOKEN`, `PINCHO_TIMEOUT`, `PINCHO_MAX_RETRIES`

## Development

```bash
cargo build               # Build
cargo test                # Run all tests (59 total)
cargo fmt                 # Format
cargo clippy              # Lint
```

## Notes

- 59 tests (37 unit + 14 integration + 8 doc)
- Bearer token auth (`Authorization: Bearer {token}`)
- Auto retry with exponential backoff (1s, 2s, 4s... capped at 30s)
- Tag normalization (lowercase, trim, dedupe, filter invalid chars)
- Encryption: AES-128-CBC with hex IV field
- Rate limit headers: `RateLimit-Limit`, `RateLimit-Remaining`, `RateLimit-Reset`
- User-Agent: `pincho-rust/{version}`
