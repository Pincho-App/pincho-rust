//! # WirePusher Rust Client Library
//!
//! Official Rust Client Library for [WirePusher](https://wirepusher.dev) - Send push notifications to your mobile devices.
//!
//! ## Features
//!
//! - **Async/await support** - Built on tokio for high-performance async operations
//! - **Type-safe builder pattern** - Compile-time guarantees for notification construction
//! - **Connection pooling** - Automatic HTTP connection reuse via reqwest
//! - **Comprehensive error handling** - Detailed error types with thiserror
//! - **Zero-copy where possible** - Efficient memory usage
//! - **Thread-safe** - All types are Send + Sync
//!
//! ## Quick Start
//!
//! ```no_run
//! use wirepusher::Client;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), wirepusher::Error> {
//!     let client = Client::new("wpt_your_token", "your_device_id")?;
//!
//!     // Send a simple notification
//!     let response = client.send("Build Complete", "v1.2.3 deployed successfully").await?;
//!
//!     if response.is_success() {
//!         println!("Notification sent successfully!");
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Advanced Usage
//!
//! ```no_run
//! use wirepusher::{Client, Notification};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), wirepusher::Error> {
//!     let client = Client::new("wpt_your_token", "your_device_id")?;
//!
//!     // Build a notification with all options
//!     let notification = Notification::builder()
//!         .title("Deploy Complete")
//!         .message("Version 1.2.3 deployed to production")
//!         .notification_type("deployment")
//!         .tags(vec!["production".to_string(), "release".to_string()])
//!         .image_url("https://example.com/success.png")
//!         .action_url("https://example.com/deployment/123")
//!         .build()?;
//!
//!     let response = client.send_notification(notification).await?;
//!     println!("Status: {}", response.status);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Error Handling
//!
//! ```no_run
//! use wirepusher::{Client, Error};
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = Client::new("wpt_token", "device_id").unwrap();
//!
//!     match client.send("Test", "Message").await {
//!         Ok(response) => println!("Success: {}", response.message),
//!         Err(Error::Authentication { message, status_code }) => {
//!             eprintln!("Auth error ({}): {}", status_code, message);
//!         }
//!         Err(Error::Validation { message, status_code }) => {
//!             eprintln!("Validation error ({}): {}", status_code, message);
//!         }
//!         Err(Error::RateLimit { message, status_code }) => {
//!             eprintln!("Rate limit ({}): {}", status_code, message);
//!         }
//!         Err(e) => eprintln!("Error: {}", e),
//!     }
//! }
//! ```
//!
//! ## Requirements
//!
//! - Rust 2021 edition or later
//! - Tokio runtime for async operations
//!
//! ## Links
//!
//! - [WirePusher Website](https://wirepusher.dev)
//! - [API Documentation](https://wirepusher.dev/api)
//! - [GitLab Repository](https://gitlab.com/wirepusher/wirepusher-rust)
//! - [Issue Tracker](https://gitlab.com/wirepusher/wirepusher-rust/-/issues)

// Module declarations
mod client;
mod encryption;
mod error;
mod notification;
mod response;

// Public re-exports
pub use client::Client;
pub use encryption::encrypt_text;
pub use error::{Error, Result};
pub use notification::{Notification, NotificationBuilder};
pub use response::NotificationResponse;
