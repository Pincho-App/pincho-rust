# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.0] - 2025-10-26

### Added
- Initial release of WirePusher Rust SDK
- `Client` struct with async `send()` and `send_notification()` methods
- Full async/await support with tokio
- Type-safe builder pattern for `Notification`
- Comprehensive error types with thiserror:
  - `Error::Authentication` - Auth failures (401, 403)
  - `Error::Validation` - Invalid parameters (400, 404)
  - `Error::RateLimit` - Rate limit errors (429)
  - `Error::Request` - Network/HTTP errors
  - `Error::Json` - JSON serialization errors
  - `Error::Api` - Other API errors
  - `Error::InvalidConfig` - Configuration errors
  - `Error::BuilderValidation` - Builder validation errors
- Connection pooling via reqwest for optimal performance
- Thread-safe implementation (all types are Send + Sync)
- Comprehensive test suite with wiremock (>90% coverage)
- Complete documentation with rustdoc comments
- Support for all v1 API parameters:
  - `title` and `message` (required)
  - `notification_type` for categorization
  - `tags` for flexible organization (up to 10 tags)
  - `image_url` for visual attachments
  - `action_url` for tap actions
- Examples:
  - `simple.rs` - Basic notification sending
  - `advanced.rs` - Full-featured notification with error handling
- Comprehensive documentation:
  - README with quickstart guide
  - API reference
  - Error handling guide
  - Usage examples (simple, advanced, concurrent)
  - Best practices
- MIT License

### Dependencies
- reqwest 0.12 (async HTTP client with JSON support)
- tokio 1.x (async runtime)
- serde 1.0 (JSON serialization)
- serde_json 1.0 (JSON handling)
- thiserror 2.0 (error handling)

### Development Dependencies
- wiremock 0.6 (HTTP mocking for tests)
- tokio-test 0.4 (testing utilities)
- futures 0.3 (concurrent testing)

[Unreleased]: https://gitlab.com/wirepusher/rust-sdk/-/compare/v1.0.0...main
[1.0.0]: https://gitlab.com/wirepusher/rust-sdk/-/tags/v1.0.0
