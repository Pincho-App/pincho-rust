use crate::error::{Error, Result};
use crate::notification::Notification;
use crate::response::{ErrorResponse, NotificationResponse, RateLimitInfo};
use reqwest::StatusCode;
use serde_json::json;
use std::sync::{Arc, RwLock};
use std::time::Duration;

/// WirePusher API client
///
/// The client is thread-safe and reuses HTTP connections via connection pooling.
/// It's recommended to create a single client instance and reuse it across your application.
///
/// # Examples
///
/// ```no_run
/// use wirepusher::Client;
///
/// #[tokio::main]
/// async fn main() -> Result<(), wirepusher::Error> {
///     let client = Client::new("abc12345")?;
///     let response = client.send("Hello", "World!").await?;
///     println!("Status: {}", response.status);
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Client {
    http_client: reqwest::Client,
    token: String,
    base_url: String,
    max_retries: u32,
    last_rate_limit: Arc<RwLock<Option<RateLimitInfo>>>,
}

impl Client {
    /// Default API base URL
    const DEFAULT_BASE_URL: &'static str = "https://api.wirepusher.dev";
    /// Default timeout in seconds
    const DEFAULT_TIMEOUT: u64 = 30;
    /// Default max retries
    const DEFAULT_MAX_RETRIES: u32 = 3;
    /// Maximum backoff delay in seconds
    const MAX_BACKOFF_SECS: u64 = 30;
    /// Library version
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    /// Creates a new WirePusher client
    ///
    /// # Arguments
    ///
    /// * `token` - Your WirePusher API token (Bearer token)
    ///
    /// # Environment Variables
    ///
    /// - `WIREPUSHER_TOKEN` - Default token if not provided
    /// - `WIREPUSHER_TIMEOUT` - Request timeout in seconds (default: 30)
    /// - `WIREPUSHER_MAX_RETRIES` - Max retry attempts (default: 3)
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Token is empty
    /// - HTTP client cannot be created
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use wirepusher::Client;
    ///
    /// # fn main() -> Result<(), wirepusher::Error> {
    /// let client = Client::new("abc12345")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(token: impl Into<String>) -> Result<Self> {
        let token = token.into();

        if token.is_empty() {
            return Err(Error::InvalidConfig("token cannot be empty".to_string()));
        }

        let timeout = std::env::var("WIREPUSHER_TIMEOUT")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(Self::DEFAULT_TIMEOUT);

        let max_retries = std::env::var("WIREPUSHER_MAX_RETRIES")
            .ok()
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(Self::DEFAULT_MAX_RETRIES);

        let http_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(timeout))
            .user_agent(format!("wirepusher-rust/{}", Self::VERSION))
            .build()
            .map_err(Error::Request)?;

        Ok(Self {
            http_client,
            token,
            base_url: Self::DEFAULT_BASE_URL.to_string(),
            max_retries,
            last_rate_limit: Arc::new(RwLock::new(None)),
        })
    }

    /// Creates a client from environment variables
    ///
    /// Reads `WIREPUSHER_TOKEN` from environment.
    ///
    /// # Errors
    ///
    /// Returns an error if `WIREPUSHER_TOKEN` is not set or empty.
    pub fn from_env() -> Result<Self> {
        let token = std::env::var("WIREPUSHER_TOKEN")
            .map_err(|_| Error::InvalidConfig("WIREPUSHER_TOKEN not set".to_string()))?;
        Self::new(token)
    }

    /// Creates a client with custom configuration
    ///
    /// # Arguments
    ///
    /// * `token` - Your WirePusher API token
    /// * `timeout` - Request timeout in seconds
    /// * `max_retries` - Maximum retry attempts for retryable errors
    pub fn with_config(token: impl Into<String>, timeout: u64, max_retries: u32) -> Result<Self> {
        let token = token.into();

        if token.is_empty() {
            return Err(Error::InvalidConfig("token cannot be empty".to_string()));
        }

        let http_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(timeout))
            .user_agent(format!("wirepusher-rust/{}", Self::VERSION))
            .build()
            .map_err(Error::Request)?;

        Ok(Self {
            http_client,
            token,
            base_url: Self::DEFAULT_BASE_URL.to_string(),
            max_retries,
            last_rate_limit: Arc::new(RwLock::new(None)),
        })
    }

    /// Creates a client with a custom base URL (useful for testing)
    ///
    /// # Arguments
    ///
    /// * `token` - Your WirePusher API token
    /// * `base_url` - Custom base URL for the API
    pub fn with_base_url(token: impl Into<String>, base_url: impl Into<String>) -> Result<Self> {
        let mut client = Self::new(token)?;
        client.base_url = base_url.into();
        Ok(client)
    }

    /// Sets a custom base URL (useful for testing)
    pub fn set_base_url(&mut self, base_url: impl Into<String>) {
        self.base_url = base_url.into();
    }

    /// Sets the max retries (useful for testing)
    pub fn set_max_retries(&mut self, max_retries: u32) {
        self.max_retries = max_retries;
    }

    /// Returns the last rate limit information
    pub fn get_last_rate_limit(&self) -> Option<RateLimitInfo> {
        self.last_rate_limit
            .read()
            .ok()
            .and_then(|guard| guard.clone())
    }

    /// Sends a simple notification with just title and message
    ///
    /// # Arguments
    ///
    /// * `title` - Notification title (max 256 characters)
    /// * `message` - Notification message (max 4096 characters)
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The HTTP request fails
    /// - The API returns an error response
    /// - The response cannot be parsed
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use wirepusher::Client;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), wirepusher::Error> {
    ///     let client = Client::new("abc12345")?;
    ///     let response = client.send("Build Complete", "v1.2.3 deployed").await?;
    ///     assert!(response.is_success());
    ///     Ok(())
    /// }
    /// ```
    pub async fn send(
        &self,
        title: impl Into<String>,
        message: impl Into<String>,
    ) -> Result<NotificationResponse> {
        let notification = Notification::new(title, message);
        self.send_notification(notification).await
    }

    /// Sends a notification with full options
    ///
    /// # Arguments
    ///
    /// * `notification` - The notification to send
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The HTTP request fails
    /// - The API returns an error response
    /// - The response cannot be parsed
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use wirepusher::{Client, Notification};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), wirepusher::Error> {
    ///     let client = Client::new("abc12345")?;
    ///
    ///     let notification = Notification::builder()
    ///         .title("Deploy Complete")
    ///         .message("v1.2.3 deployed to production")
    ///         .notification_type("deployment")
    ///         .tags(vec!["prod".to_string(), "release".to_string()])
    ///         .build()?;
    ///
    ///     let response = client.send_notification(notification).await?;
    ///     assert!(response.is_success());
    ///     Ok(())
    /// }
    /// ```
    pub async fn send_notification(
        &self,
        mut notification: Notification,
    ) -> Result<NotificationResponse> {
        // Normalize tags
        if let Some(ref mut tags) = notification.tags {
            *tags = Self::normalize_tags(tags);
        }

        // Handle encryption if password is set
        let message = if let Some(ref password) = notification.encryption_password {
            // Generate random IV
            use rand::Rng;
            let mut iv = [0u8; 16];
            rand::thread_rng().fill(&mut iv);

            // Encrypt the message
            let encrypted = crate::encryption::encrypt_text(&notification.message, password, &iv)?;

            // Store IV as hex (matching other SDKs)
            notification.message_iv = Some(hex::encode(iv));

            encrypted
        } else {
            notification.message.clone()
        };

        // Build the request body with camelCase field names
        let mut body = json!({
            "title": notification.title,
            "message": message,
        });

        // Add optional fields with camelCase
        if let Some(notification_type) = notification.notification_type {
            body["type"] = json!(notification_type);
        }
        if let Some(tags) = notification.tags {
            body["tags"] = json!(tags);
        }
        if let Some(image_url) = notification.image_url {
            body["imageURL"] = json!(image_url);
        }
        if let Some(action_url) = notification.action_url {
            body["actionURL"] = json!(action_url);
        }
        if let Some(message_iv) = notification.message_iv {
            body["iv"] = json!(message_iv);
        }

        // Retry loop
        let mut last_error: Option<Error> = None;
        for attempt in 0..=self.max_retries {
            if attempt > 0 {
                // Exponential backoff: 1s, 2s, 4s, 8s... capped at 30s
                let delay_secs = std::cmp::min(1u64 << (attempt - 1), Self::MAX_BACKOFF_SECS);
                tokio::time::sleep(Duration::from_secs(delay_secs)).await;
            }

            // Send the request
            let response = match self
                .http_client
                .post(format!("{}/send", self.base_url))
                .bearer_auth(&self.token)
                .json(&body)
                .send()
                .await
            {
                Ok(resp) => resp,
                Err(e) => {
                    let error = Error::Request(e);
                    if error.is_retryable() && attempt < self.max_retries {
                        last_error = Some(error);
                        continue;
                    }
                    return Err(error);
                }
            };

            // Parse rate limit headers
            self.parse_rate_limit_headers(&response);

            let status = response.status();

            // Parse error responses
            if !status.is_success() {
                let error_text = response.text().await.unwrap_or_default();
                let error = self.parse_error_response(status, &error_text);

                if error.is_retryable() && attempt < self.max_retries {
                    last_error = Some(error);
                    continue;
                }
                return Err(error);
            }

            // Parse success response
            let notification_response: NotificationResponse = response.json().await?;
            return Ok(notification_response);
        }

        // Should not reach here, but return last error if we do
        Err(last_error.unwrap_or_else(|| Error::Api {
            message: "Max retries exceeded".to_string(),
            status_code: 0,
        }))
    }

    /// Normalizes tags to match API expectations
    fn normalize_tags(tags: &[String]) -> Vec<String> {
        tags.iter()
            .map(|tag| {
                tag.trim()
                    .to_lowercase()
                    .chars()
                    .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
                    .collect::<String>()
            })
            .filter(|tag| !tag.is_empty())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect()
    }

    /// Parses rate limit headers from response
    fn parse_rate_limit_headers(&self, response: &reqwest::Response) {
        let headers = response.headers();

        let limit = headers
            .get("RateLimit-Limit")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<u32>().ok());

        let remaining = headers
            .get("RateLimit-Remaining")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<u32>().ok());

        let reset = headers
            .get("RateLimit-Reset")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<u64>().ok());

        if let (Some(limit), Some(remaining), Some(reset)) = (limit, remaining, reset) {
            let rate_limit_info = RateLimitInfo {
                limit,
                remaining,
                reset,
            };
            if let Ok(mut guard) = self.last_rate_limit.write() {
                *guard = Some(rate_limit_info);
            }
        }
    }

    /// Sends a notification using NotifAI (AI-powered notification generation)
    ///
    /// # Arguments
    ///
    /// * `text` - Free-form text describing what happened
    /// * `notification_type` - Optional notification type for categorization
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use wirepusher::Client;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), wirepusher::Error> {
    ///     let client = Client::new("abc12345")?;
    ///     let response = client.notifai("deployment finished v2.1.3", None).await?;
    ///     println!("Generated title: {}", response.notification.title);
    ///     Ok(())
    /// }
    /// ```
    pub async fn notifai(
        &self,
        text: impl Into<String>,
        notification_type: Option<String>,
    ) -> Result<crate::response::NotifAIResponse> {
        let text = text.into();

        // Build the request body
        let mut body = json!({
            "text": text,
        });

        if let Some(t) = notification_type {
            body["type"] = json!(t);
        }

        // Retry loop
        let mut last_error: Option<Error> = None;
        for attempt in 0..=self.max_retries {
            if attempt > 0 {
                let delay_secs = std::cmp::min(1u64 << (attempt - 1), Self::MAX_BACKOFF_SECS);
                tokio::time::sleep(Duration::from_secs(delay_secs)).await;
            }

            // Send the request
            let response = match self
                .http_client
                .post(format!("{}/notifai", self.base_url))
                .bearer_auth(&self.token)
                .json(&body)
                .send()
                .await
            {
                Ok(resp) => resp,
                Err(e) => {
                    let error = Error::Request(e);
                    if error.is_retryable() && attempt < self.max_retries {
                        last_error = Some(error);
                        continue;
                    }
                    return Err(error);
                }
            };

            // Parse rate limit headers
            self.parse_rate_limit_headers(&response);

            let status = response.status();

            if !status.is_success() {
                let error_text = response.text().await.unwrap_or_default();
                let error = self.parse_error_response(status, &error_text);

                if error.is_retryable() && attempt < self.max_retries {
                    last_error = Some(error);
                    continue;
                }
                return Err(error);
            }

            // Parse success response
            let notifai_response: crate::response::NotifAIResponse = response.json().await?;
            return Ok(notifai_response);
        }

        Err(last_error.unwrap_or_else(|| Error::Api {
            message: "Max retries exceeded".to_string(),
            status_code: 0,
        }))
    }

    /// Parses error response with nested error format
    fn parse_error_response(&self, status: StatusCode, error_text: &str) -> Error {
        let status_code = status.as_u16();

        // Parse nested error response format (required)
        let error_message = match serde_json::from_str::<ErrorResponse>(error_text) {
            Ok(error_resp) => {
                // Build descriptive error message from nested error details
                let mut msg = error_resp.error.message.clone();

                // Add error code if available and non-empty
                if !error_resp.error.code.is_empty() {
                    msg = format!("{} [{}]", msg, error_resp.error.code);
                }

                // Add parameter context if available
                if let Some(param) = &error_resp.error.param {
                    if !param.is_empty() {
                        msg = format!("{} (parameter: {})", msg, param);
                    }
                }

                msg
            }
            Err(_) => {
                // Return generic error if nested format is not present
                format!("API error: HTTP {}", status_code)
            }
        };

        // Map to specific error type based on status code
        match status {
            StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => Error::Authentication {
                message: error_message,
                status_code,
            },
            StatusCode::BAD_REQUEST | StatusCode::NOT_FOUND => Error::Validation {
                message: error_message,
                status_code,
            },
            StatusCode::TOO_MANY_REQUESTS => Error::RateLimit {
                message: error_message,
                status_code,
            },
            _ => Error::Api {
                message: error_message,
                status_code,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_new() {
        let client = Client::new("abc12345").unwrap();
        assert_eq!(client.token, "abc12345");
        assert_eq!(client.base_url, Client::DEFAULT_BASE_URL);
        assert_eq!(client.max_retries, Client::DEFAULT_MAX_RETRIES);
    }

    #[test]
    fn test_client_with_custom_base_url() {
        let client = Client::with_base_url("abc12345", "https://custom.api.com").unwrap();
        assert_eq!(client.base_url, "https://custom.api.com");
    }

    #[test]
    fn test_client_with_config() {
        let client = Client::with_config("abc12345", 60, 5).unwrap();
        assert_eq!(client.token, "abc12345");
        assert_eq!(client.max_retries, 5);
    }

    #[test]
    fn test_client_empty_token() {
        let result = Client::new("");
        assert!(result.is_err());
        assert!(matches!(result, Err(Error::InvalidConfig(_))));
    }

    #[test]
    fn test_normalize_tags() {
        let tags = vec![
            "  Production  ".to_string(),
            "BACKEND".to_string(),
            "test-tag".to_string(),
            "special@chars!".to_string(),
            "  ".to_string(), // Empty after trim
        ];

        let normalized = Client::normalize_tags(&tags);

        assert!(normalized.contains(&"production".to_string()));
        assert!(normalized.contains(&"backend".to_string()));
        assert!(normalized.contains(&"test-tag".to_string()));
        assert!(normalized.contains(&"specialchars".to_string()));
        assert!(!normalized.iter().any(|t| t.is_empty()));
    }

    #[test]
    fn test_normalize_tags_deduplication() {
        let tags = vec!["test".to_string(), "TEST".to_string(), "Test".to_string()];

        let normalized = Client::normalize_tags(&tags);
        assert_eq!(normalized.len(), 1);
        assert!(normalized.contains(&"test".to_string()));
    }

    #[test]
    fn test_parse_nested_error_format() {
        let client = Client::new("token").unwrap();
        let nested_error = r#"{"status":"error","error":{"type":"validation_error","code":"invalid_parameter","message":"Title is required","param":"title"}}"#;
        let error = client.parse_error_response(StatusCode::BAD_REQUEST, nested_error);

        assert!(error.is_validation_error());
        match error {
            Error::Validation {
                message,
                status_code,
            } => {
                assert_eq!(status_code, 400);
                assert!(message.contains("Title is required"));
                assert!(message.contains("[invalid_parameter]"));
                assert!(message.contains("(parameter: title)"));
            }
            _ => panic!("Expected Validation error"),
        }
    }

    #[test]
    fn test_parse_nested_error_without_param() {
        let client = Client::new("token").unwrap();
        let nested_error = r#"{"status":"error","error":{"type":"authentication_error","code":"invalid_token","message":"Token is invalid"}}"#;
        let error = client.parse_error_response(StatusCode::UNAUTHORIZED, nested_error);

        assert!(error.is_authentication_error());
        match error {
            Error::Authentication {
                message,
                status_code,
            } => {
                assert_eq!(status_code, 401);
                assert!(message.contains("Token is invalid"));
                assert!(message.contains("[invalid_token]"));
                assert!(!message.contains("(parameter:"));
            }
            _ => panic!("Expected Authentication error"),
        }
    }
}
