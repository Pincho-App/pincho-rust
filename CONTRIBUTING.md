# Contributing to WirePusher Rust Client Library

Thanks for considering contributing! This is a small project with a small team, so every contribution makes a real difference.

## Code of Conduct

Be respectful and constructive. See [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) for details.

## Quick Start

```bash
# Get the code
git clone https://gitlab.com/wirepusher/wirepusher-rust.git
cd wirepusher-rust

# Build
cargo build

# Run tests
cargo test

# Make sure everything passes
cargo test
cargo fmt -- --check
cargo clippy -- -D warnings
```

**Requirements:** Rust 2021 edition or later

## How to Contribute

### Report a Bug

[Check existing issues](https://gitlab.com/wirepusher/wirepusher-rust/-/issues) first, then create a new one with:

- What you did (exact code)
- What happened vs what you expected
- Your environment (SDK version, Rust version, OS)
- Error output

**Example:**
```
## Encryption doesn't work with special characters

**Code:**
```rust
use wirepusher::{Client, Notification};

let client = Client::new("wpt_abc123", "device_id").unwrap();
let notification = Notification::builder()
    .title("Test")
    .message("Special: ñ € 中")
    .notification_type("test")
    .encryption_password("secret")
    .build()?;
client.send_notification(notification).await?;
```

**Expected:** Message decrypts correctly
**Actual:** Message appears garbled
**Environment:** SDK v1.0.0, Rust 1.75, macOS 14.0

[Paste error output]
```

### Suggest a Feature

Create an issue explaining:
- What you want and why
- How it would work
- Any alternatives you considered

### Submit Code

1. **Fork** the repo
2. **Create a branch**: `git checkout -b fix-something`
3. **Make your changes**
4. **Add tests** for new functionality
5. **Run tests**: `cargo test`
6. **Format**: `cargo fmt && cargo clippy -- -D warnings`
7. **Commit** with a clear message
8. **Push** to your fork
9. **Open a Merge Request**

## Code Guidelines

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use type hints and rustdoc comments
- Write clear documentation
- Keep functions small and focused
- Update README for user-facing changes

**Good commit messages:**
```
Add retry logic for network errors
Fix tag validation edge case
Update docs with encryption examples
```

**Bad commit messages:**
```
fix bug
update
changes
```

## Project Structure

```
src/          # Library code (client, notification, error, encryption, response)
tests/        # Integration tests (wiremock-based HTTP mocking)
examples/     # Usage examples
docs/         # Documentation
```

See [CLAUDE.md](CLAUDE.md) for details.

## Common Tasks

**Add a feature:** Update `client.rs` or `notification.rs`, add tests, write rustdoc comments

**Add validation:** Add logic in `notification.rs` builder, write tests, integrate into client

**Add error type:** Add to `error.rs` enum, update error handling, export in `lib.rs`

## Testing

```bash
cargo test                          # All tests
cargo test --verbose               # Verbose
cargo test -- --nocapture          # Show output
cargo test test_send_success       # Specific test
```

## Code Quality

```bash
# Formatting
cargo fmt

# Linting
cargo clippy -- -D warnings

# Documentation
cargo doc --open

# All checks
cargo test && cargo fmt -- --check && cargo clippy -- -D warnings
```

## Need Help?

- **Architecture questions?** See [CLAUDE.md](CLAUDE.md)
- **Stuck?** Open an issue or ask in your MR

## License

By contributing, you agree your contributions will be licensed under the MIT License.

---

Thanks for contributing!
