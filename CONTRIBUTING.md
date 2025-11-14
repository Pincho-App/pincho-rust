# Contributing to WirePusher Rust Client Library

Thank you for your interest in contributing to the WirePusher Rust Client Library! This document provides guidelines and instructions for contributing.

## Code of Conduct

This project adheres to a Code of Conduct. By participating, you are expected to uphold this code. Please be respectful and professional in all interactions.

## Getting Started

### Prerequisites

- Rust (stable, 2021 edition or later)
- Cargo
- Git

### Setting Up Development Environment

1. Fork the repository on GitLab
2. Clone your fork:
   ```bash
   git clone https://gitlab.com/your-username/rust-sdk.git
   cd rust-sdk
   ```
3. Add upstream remote:
   ```bash
   git remote add upstream https://gitlab.com/wirepusher/wirepusher-rust.git
   ```
4. Install dependencies:
   ```bash
   cargo build
   ```

## Development Workflow

### Making Changes

1. Create a new branch:
   ```bash
   git checkout -b feature/my-new-feature
   ```

2. Make your changes following the coding standards below

3. Run tests:
   ```bash
   cargo test
   ```

4. Check code formatting:
   ```bash
   cargo fmt -- --check
   ```

5. Run linter:
   ```bash
   cargo clippy -- -D warnings
   ```

6. Commit your changes:
   ```bash
   git commit -m "feat: add new feature"
   ```

### Commit Message Convention

Follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation changes
- `test:` - Test changes
- `refactor:` - Code refactoring
- `perf:` - Performance improvements
- `chore:` - Maintenance tasks

### Submitting Pull Requests

1. Push your branch:
   ```bash
   git push origin feature/my-new-feature
   ```

2. Open a Merge Request on GitLab

3. Ensure the MR description includes:
   - What changes were made
   - Why the changes were necessary
   - Any relevant issue numbers

4. Wait for review from maintainers

## Coding Standards

### Rust Style Guide

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for consistent formatting
- Run `cargo clippy` and address all warnings
- Write idiomatic Rust code

### Documentation

- Add rustdoc comments for all public APIs
- Include examples in documentation comments
- Keep README.md up to date
- Update CHANGELOG.md following Keep a Changelog format

### Testing

- Write unit tests for all new functionality
- Maintain >90% code coverage
- Use wiremock for HTTP mocking in tests
- Add integration tests for major features
- Include doc tests in rustdoc comments

### Error Handling

- Use the existing `Error` enum for all errors
- Add new error variants if needed
- Provide clear, actionable error messages
- Include context in error propagation

## Project Structure

```
rust-sdk/
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
├── Cargo.toml
└── README.md
```

## Testing

### Running Tests

```bash
# All tests
cargo test

# Specific test
cargo test test_send_simple_notification_success

# With output
cargo test -- --nocapture
```

### Code Coverage

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage
cargo tarpaulin --out Html --output-dir coverage

# Open coverage report
open coverage/index.html
```

## Documentation

### Building Documentation

```bash
cargo doc --open
```

### Writing Documentation

- Use `///` for public items
- Use `//!` for module-level documentation
- Include code examples in doc comments
- Use doc tests: ` ```rust`

Example:

```rust
/// Sends a notification
///
/// # Examples
///
/// ```rust
/// use wirepusher::Client;
///
/// #[tokio::main]
/// async fn main() -> Result<(), wirepusher::Error> {
///     let client = Client::new("token", "user_id")?;
///     client.send("Title", "Message").await?;
///     Ok(())
/// }
/// ```
pub async fn send(&self, title: &str, message: &str) -> Result<NotificationResponse>
```

## Release Process

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md` with release notes
3. Commit changes: `git commit -m "chore: release v1.x.x"`
4. Create tag: `git tag v1.x.x`
5. Push: `git push && git push --tags`
6. Publish to crates.io: `cargo publish`

## Questions?

- Open an issue on GitLab
- Email: support@wirepusher.dev

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
