use crate::error::{Error, Result};
use crate::notification::Notification;
use crate::response::NotificationResponse;
use reqwest::StatusCode;
use serde_json::json;

/// WirePusher API client
///
/// The client is thread-safe and reuses HTTP connections via connection pooling.
/// It's recommended to create a single client instance and reuse it across your application.
///
/// # Deprecation Note
///
/// This client uses the v1 API with legacy `device_id` authentication. For new projects,
/// consider using Firebase Auth tokens with the v2 API when available.
///
/// # Examples
///
/// ```no_run
/// use wirepusher::Client;
///
/// #[tokio::main]
/// async fn main() -> Result<(), wirepusher::Error> {
///     let client = Client::new("wpt_your_token", "your_device_id")?;
///     let response = client.send("Hello", "World!").await?;
///     println!("Status: {}", response.status);
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Client {
    http_client: reqwest::Client,
    token: String,
    /// Legacy authentication - device ID (deprecated, use token-based auth for new projects)
    #[deprecated(note = "Legacy authentication. Use token parameter instead with v2 API.")]
    device_id: String,
    base_url: String,
}

impl Client {
    /// Default API base URL
    const DEFAULT_BASE_URL: &'static str = "https://wirepusher.com/send";

    /// Creates a new WirePusher client
    ///
    /// # Arguments
    ///
    /// * `token` - Your WirePusher API token (starts with `wpt_`)
    /// * `device_id` - Your WirePusher device ID
    ///
    /// # Deprecation Warning
    ///
    /// The `device_id` parameter is legacy authentication. For new integrations, use Firebase Auth tokens
    /// via the v2 API instead. This SDK currently only supports v1 API with device_id authentication.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Token is empty
    /// - Device ID is empty
    /// - HTTP client cannot be created
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use wirepusher::Client;
    ///
    /// # fn main() -> Result<(), wirepusher::Error> {
    /// let client = Client::new("wpt_your_token", "your_device_id")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(token: impl Into<String>, device_id: impl Into<String>) -> Result<Self> {
        let token = token.into();
        let device_id = device_id.into();

        if token.is_empty() {
            return Err(Error::InvalidConfig("token cannot be empty".to_string()));
        }

        if device_id.is_empty() {
            return Err(Error::InvalidConfig(
                "device_id cannot be empty".to_string(),
            ));
        }

        let http_client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(|e| Error::Request(e))?;

        Ok(Self {
            http_client,
            token,
            device_id,
            base_url: Self::DEFAULT_BASE_URL.to_string(),
        })
    }

    /// Creates a client with a custom base URL (useful for testing)
    ///
    /// # Arguments
    ///
    /// * `token` - Your WirePusher API token
    /// * `device_id` - Your WirePusher device ID (legacy authentication)
    /// * `base_url` - Custom base URL for the API
    ///
    /// # Deprecation Warning
    ///
    /// The `device_id` parameter is legacy authentication. Use token-based auth for new integrations.
    pub fn with_base_url(
        token: impl Into<String>,
        device_id: impl Into<String>,
        base_url: impl Into<String>,
    ) -> Result<Self> {
        let mut client = Self::new(token, device_id)?;
        client.base_url = base_url.into();
        Ok(client)
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
    ///     let client = Client::new("wpt_token", "device_id")?;
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
    ///     let client = Client::new("wpt_token", "device_id")?;
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
        // Handle encryption if password is set
        let message = if let Some(ref password) = notification.encryption_password {
            // Generate random IV
            use rand::Rng;
            let mut iv = [0u8; 16];
            rand::thread_rng().fill(&mut iv);

            // Encrypt the message
            let encrypted = crate::encryption::encrypt_text(&notification.message, password, &iv)?;

            // Store IV in custom Base64 format
            use base64::{engine::general_purpose::STANDARD, Engine as _};
            let iv_base64 = STANDARD.encode(iv);
            notification.message_iv = Some(
                iv_base64
                    .replace('+', "-")
                    .replace('/', ".")
                    .replace('=', "_"),
            );

            encrypted
        } else {
            notification.message.clone()
        };

        // Build the request body
        let mut body = json!({
            "id": self.device_id,
            "title": notification.title,
            "message": message,
        });

        // Add optional fields
        if let Some(notification_type) = notification.notification_type {
            body["type"] = json!(notification_type);
        }
        if let Some(tags) = notification.tags {
            body["tags"] = json!(tags);
        }
        if let Some(image_url) = notification.image_url {
            body["image_url"] = json!(image_url);
        }
        if let Some(action_url) = notification.action_url {
            body["action_url"] = json!(action_url);
        }
        if let Some(message_iv) = notification.message_iv {
            body["message_iv"] = json!(message_iv);
        }

        // Send the request
        let response = self
            .http_client
            .post(&self.base_url)
            .query(&[("type", "wirepusher"), ("token", &self.token)])
            .json(&body)
            .send()
            .await?;

        let status = response.status();

        // Parse error responses
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(self.map_error(status, error_text));
        }

        // Parse success response
        let notification_response: NotificationResponse = response.json().await?;
        Ok(notification_response)
    }

    /// Maps HTTP status codes to specific error types
    fn map_error(&self, status: StatusCode, message: String) -> Error {
        let status_code = status.as_u16();

        match status {
            StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => Error::Authentication {
                message,
                status_code,
            },
            StatusCode::BAD_REQUEST | StatusCode::NOT_FOUND => Error::Validation {
                message,
                status_code,
            },
            StatusCode::TOO_MANY_REQUESTS => Error::RateLimit {
                message,
                status_code,
            },
            _ => Error::Api {
                message,
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
        let client = Client::new("wpt_test_token", "test_device_id").unwrap();
        assert_eq!(client.token, "wpt_test_token");
        assert_eq!(client.device_id, "test_device_id");
        assert_eq!(client.base_url, Client::DEFAULT_BASE_URL);
    }

    #[test]
    fn test_client_with_custom_base_url() {
        let client =
            Client::with_base_url("wpt_token", "device_id", "https://custom.api.com/send").unwrap();
        assert_eq!(client.base_url, "https://custom.api.com/send");
    }

    #[test]
    fn test_client_empty_token() {
        let result = Client::new("", "device_id");
        assert!(result.is_err());
        assert!(matches!(result, Err(Error::InvalidConfig(_))));
    }

    #[test]
    fn test_client_empty_device_id() {
        let result = Client::new("wpt_token", "");
        assert!(result.is_err());
        assert!(matches!(result, Err(Error::InvalidConfig(_))));
    }

    #[test]
    fn test_map_error_authentication() {
        let client = Client::new("token", "device_id").unwrap();
        let error = client.map_error(StatusCode::UNAUTHORIZED, "Invalid token".to_string());
        assert!(error.is_authentication_error());
    }

    #[test]
    fn test_map_error_validation() {
        let client = Client::new("token", "device_id").unwrap();
        let error = client.map_error(StatusCode::BAD_REQUEST, "Invalid params".to_string());
        assert!(error.is_validation_error());
    }

    #[test]
    fn test_map_error_rate_limit() {
        let client = Client::new("token", "device_id").unwrap();
        let error = client.map_error(StatusCode::TOO_MANY_REQUESTS, "Rate limit".to_string());
        assert!(error.is_rate_limit_error());
    }
}
