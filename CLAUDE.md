# CLAUDE.md - WirePusher Rust Client Library

Context file for AI-powered development assistance on the WirePusher Rust Client Library project.

## Project Overview

**WirePusher Rust Client Library** is a Rust client library for sending push notifications via [WirePusher](https://wirepusher.dev).

- **Language**: Rust 2021 edition
- **HTTP Client**: reqwest 0.12 (async HTTP client with HTTP/2 support)
- **Runtime**: tokio 1.x (async runtime)
- **Purpose**: Send notifications from Rust applications and services
- **Philosophy**: Type-safe, async-first, zero-cost abstractions

## Architecture

```
wirepusher-rust/
├── src/                      # Main library code
│   ├── lib.rs                # Public API exports
│   ├── client.rs             # Client implementation
│   ├── notification.rs       # Notification struct + Builder pattern
│   ├── response.rs           # NotificationResponse struct
│   ├── error.rs              # Error types (thiserror)
│   └── encryption.rs         # AES-128-CBC encryption
├── tests/                    # Integration tests
│   └── integration_tests.rs  # Wiremock-based HTTP mocking
├── examples/                 # Usage examples
│   ├── simple.rs             # Basic notification example
│   └── advanced.rs           # Full-featured example
├── docs/                     # Documentation (empty, for future guides)
└── Cargo.toml                # Package manifest

```

## Key Features

### 1. Async/Await Architecture

**Built on Tokio**:
- Full async/await support with tokio runtime
- Non-blocking I/O for high-performance operations
- Efficient connection pooling via reqwest
- All operations are async (no sync wrapper)

**Thread-Safe**:
- All types implement `Send + Sync`
- Client can be cloned and shared across threads
- Safe concurrent use without additional locking

### 2. Type-Safe Builder Pattern

**Notification Builder**:
```rust
Notification::builder()
    .title("Deploy Complete")
    .message("v1.2.3 deployed")
    .notification_type("deployment")
    .tags(vec!["production".to_string(), "release".to_string()])
    .image_url("https://example.com/image.png")
    .action_url("https://example.com/action")
    .encryption_password("secret")  // Optional
    .build()?  // Compile-time validation
```

**Benefits**:
- Compile-time guarantees for required fields
- Validation at build time (URLs, tags, lengths)
- Flexible, ergonomic API
- Prevents invalid state

### 3. API Endpoints

**send()** - Simple convenience method:
```rust
client.send("Title", "Message").await?
```

**send_notification()** - Full-featured method:
```rust
let notification = Notification::builder()
    .title("Deploy Complete")
    .message("v1.2.3 deployed to production")
    .notification_type("deployment")
    .tags(vec!["production".to_string()])
    .build()?;

client.send_notification(notification).await?
```

### 4. Comprehensive Error Types

All errors use thiserror for ergonomic error handling:

- `Error::Authentication { message, status_code }` - Auth failures (401, 403)
- `Error::Validation { message, status_code }` - Invalid parameters (400, 404)
- `Error::RateLimit { message, status_code }` - Rate limit errors (429)
- `Error::Request(reqwest::Error)` - Network/HTTP errors
- `Error::Json(serde_json::Error)` - JSON serialization errors
- `Error::Api { message, status_code }` - Other API errors
- `Error::InvalidConfig(String)` - Configuration errors
- `Error::BuilderValidation(String)` - Builder validation errors

**Helper Methods**:
```rust
error.status_code() -> Option<u16>
error.is_authentication_error() -> bool
error.is_validation_error() -> bool
error.is_rate_limit_error() -> bool
```

### 5. Tag Validation

Automatic validation in builder:
- Maximum 10 tags
- Each tag ≤50 characters
- No empty tags
- Validated at build time

Implementation: `notification.rs` (builder validation)

### 6. Encryption

**AES-128-CBC** encryption matching mobile app:
- SHA1-based key derivation (for app compatibility)
- Custom Base64 encoding (URL-safe: `-` `_` `.` instead of `+` `/` `=`)
- Only message encrypted (title, type, tags remain visible)
- Password must match type configuration in app

Implementation: `encryption.rs`

### 7. Connection Pooling

**Automatic via reqwest**:
- HTTP connection reuse
- HTTP/2 support
- Efficient resource management
- Configurable timeout (30 seconds default)

## Configuration

Simple constructor-based configuration:

```rust
// Required parameters
let client = Client::new("abc12345", "device_id")?;

// Client uses sensible defaults:
// - Timeout: 30 seconds
// - Connection pooling: enabled
// - HTTP/2: enabled
```

No config file support (not idiomatic for Rust libraries).

## Dependencies

**Runtime** (minimal):
- `reqwest = { version = "0.12", features = ["json"] }` - HTTP client
- `tokio = { version = "1", features = ["full"] }` - Async runtime
- `serde = { version = "1.0", features = ["derive"] }` - Serialization
- `serde_json = "1.0"` - JSON handling
- `thiserror = "2.0"` - Error handling
- `aes = "0.8"` - AES encryption
- `cbc = "0.1"` - CBC mode
- `sha1 = "0.10"` - SHA1 hashing
- `base64 = "0.22"` - Base64 encoding
- `rand = "0.8"` - Random IV generation
- `url = "2.5"` - URL parsing/validation

**Development**:
- `wiremock = "0.5"` - HTTP mocking for tests
- `tokio-test = "0.4"` - Testing utilities
- `futures = "0.3"` - Concurrent testing

## Recent Changes

### v1.0.0 (Current)

**Added**:
- Initial release of WirePusher Rust Client Library
- Full async/await support with tokio
- Type-safe builder pattern for Notification
- Comprehensive error types with thiserror
- AES-128-CBC encryption support
- Connection pooling via reqwest
- Thread-safe implementation (Send + Sync)
- Comprehensive test suite with wiremock (>90% coverage)
- Complete rustdoc documentation
- Examples: simple.rs, advanced.rs

**Supported Features**:
- Token-based authentication only (no device_id/user_id variants)
- All v1 API parameters (title, message, type, tags, image_url, action_url)
- Encryption support
- Error categorization
- URL validation
- Tag validation

## Development

### Setup

```bash
# Clone repository
git clone https://gitlab.com/wirepusher/wirepusher-rust.git
cd wirepusher-rust

# Build
cargo build

# Run tests
cargo test

# Check formatting
cargo fmt -- --check

# Run linter
cargo clippy -- -D warnings
```

### Testing

```bash
cargo test                          # All tests
cargo test --verbose               # Verbose output
cargo test -- --nocapture          # Show println! output
cargo test test_send_success       # Specific test

# Coverage (requires cargo-tarpaulin)
cargo install cargo-tarpaulin
cargo tarpaulin --out Html --output-dir coverage
```

### Code Quality

```bash
cargo fmt                    # Format code
cargo clippy -- -D warnings  # Lint (treat warnings as errors)
cargo doc --open             # Generate and open documentation
```

### Documentation

Rust uses rustdoc for documentation:
- Use `///` for public items
- Use `//!` for module-level documentation
- Include code examples in doc comments
- Examples are tested automatically

Example:
```rust
/// Sends a notification to WirePusher.
///
/// # Examples
///
/// ```
/// use wirepusher::Client;
///
/// #[tokio::main]
/// async fn main() -> Result<(), wirepusher::Error> {
///     let client = Client::new("abc12345", "device_id")?;
///     client.send("Title", "Message").await?;
///     Ok(())
/// }
/// ```
pub async fn send(&self, title: impl Into<String>, message: impl Into<String>) -> Result<NotificationResponse>
```

## Common Development Tasks

### Adding a Feature

1. Implement in appropriate module (`client.rs`, `notification.rs`, etc.)
2. Add error variants if needed in `error.rs`
3. Write tests in `tests/integration_tests.rs`
4. Add rustdoc comments with examples
5. Update README with usage examples
6. Add to CHANGELOG

### Adding an Error Type

1. Add variant to `Error` enum in `error.rs`
2. Implement Display/Debug if needed (thiserror handles this)
3. Add helper method if applicable (e.g., `is_xxx_error()`)
4. Update error handling in `client.rs`
5. Export in `lib.rs` if needed
6. Update README error handling section

### Adding Validation

1. Add validation logic in builder (`notification.rs`)
2. Write comprehensive tests
3. Return `BuilderValidation` error for validation failures
4. Document validation rules in rustdoc
5. Update README with examples

## Testing Philosophy

- **Integration tests**: Test HTTP interactions with wiremock
- **Unit tests**: Test individual functions/methods
- **Doc tests**: Examples in rustdoc comments are tested
- **Coverage target**: >90% for critical paths
- **Mock HTTP**: Use wiremock for HTTP mocking (no real API calls in tests)

## API Integration

### Endpoints

- `POST /send` - Send notifications

### Authentication

Token-based only:
- Token passed in constructor
- Sent as `token` parameter in API request

### Request Format

**Send request:**
```json
{
  "token": "abc12345",
  "device_id": "your_device_id",
  "title": "Deploy Complete",
  "message": "v1.2.3 deployed",
  "type": "deployment",
  "tags": ["production", "release"],
  "image_url": "https://example.com/image.png",
  "action_url": "https://example.com/action"
}
```

### Response Format

**Success response:**
```json
{
  "status": "success",
  "message": "Notification sent successfully"
}
```

**Error response:**
```json
{
  "status": "error",
  "error": {
    "type": "validation_error",
    "code": "missing_required_field",
    "message": "Title is required",
    "param": "title"
  }
}
```

The library parses error responses and maps them to appropriate Error variants.

## Notes for AI Assistants

- **Type safety is paramount**: Leverage Rust's type system for compile-time guarantees
- **Async-first**: All operations are async (no sync wrapper like Python)
- **Idiomatic Rust**: Follow Rust API Guidelines and community conventions
- **Zero config**: No config files (not idiomatic for Rust libraries)
- **Minimal dependencies**: Only essential crates (reqwest, tokio, serde, thiserror)
- **Error handling**: Use thiserror for ergonomic error types
- **Documentation**: Always include rustdoc comments with examples
- **Test coverage**: Aim for >90% on critical paths
- **Builder pattern**: Use for complex types (like Notification)
- **Consistency**: Match Python/Go features when applicable, but be Rust-idiomatic

## Project Status

**Current**: Production-ready v1.0.0

**Completed**:
- Full async/await support
- Type-safe builder pattern
- Comprehensive error types
- AES-128-CBC encryption
- Connection pooling
- Thread-safe implementation
- Comprehensive documentation
- >90% test coverage

**Not Implemented** (not needed/planned):
- Sync wrapper (async is standard in Rust)
- Config file support (not idiomatic)
- CLI tool (separate wirepusher-cli project)
- Retry logic (users should implement with backoff crates)
- NotifAI endpoint (not yet available in v1 API)

## Links

- **Repository**: https://gitlab.com/wirepusher/wirepusher-rust
- **Issues**: https://gitlab.com/wirepusher/wirepusher-rust/-/issues
- **Crates.io**: https://crates.io/crates/wirepusher
- **Documentation**: https://docs.rs/wirepusher
- **API Docs**: https://wirepusher.com/docs
- **App**: https://wirepusher.dev
