use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use url::Url;

/// A notification to be sent via Pincho
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Notification {
    /// Notification title (max 256 characters)
    pub title: String,

    /// Notification message (max 4096 characters)
    pub message: String,

    /// Notification type for categorization (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub notification_type: Option<String>,

    /// Tags for filtering (max 10, optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,

    /// URL to an image to display (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "image_url")]
    pub image_url: Option<String>,

    /// URL to open when notification is tapped (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "action_url")]
    pub action_url: Option<String>,

    /// Encryption password (optional, will encrypt message field only)
    #[serde(skip_serializing)]
    pub(crate) encryption_password: Option<String>,

    /// Random IV for encryption (optional, will be generated if encryption is used)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "message_iv")]
    pub(crate) message_iv: Option<String>,
}

impl Notification {
    /// Creates a new simple notification with just title and message
    pub fn new(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            message: message.into(),
            notification_type: None,
            tags: None,
            image_url: None,
            action_url: None,
            encryption_password: None,
            message_iv: None,
        }
    }

    /// Creates a builder for constructing a notification
    pub fn builder() -> NotificationBuilder {
        NotificationBuilder::default()
    }
}

/// Builder for creating a Notification
#[derive(Debug, Default)]
pub struct NotificationBuilder {
    title: Option<String>,
    message: Option<String>,
    notification_type: Option<String>,
    tags: Option<Vec<String>>,
    image_url: Option<String>,
    action_url: Option<String>,
    encryption_password: Option<String>,
}

impl NotificationBuilder {
    /// Sets the notification title (required, max 256 characters)
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the notification message (required, max 4096 characters)
    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    /// Sets the notification type for categorization
    pub fn notification_type(mut self, notification_type: impl Into<String>) -> Self {
        self.notification_type = Some(notification_type.into());
        self
    }

    /// Sets tags for filtering (max 10 tags, max 50 characters per tag)
    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.tags = Some(tags);
        self
    }

    /// Adds a single tag (max 50 characters)
    pub fn add_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.get_or_insert_with(Vec::new).push(tag.into());
        self
    }

    /// Sets the image URL to display (must be a valid URL)
    pub fn image_url(mut self, url: impl Into<String>) -> Self {
        self.image_url = Some(url.into());
        self
    }

    /// Sets the action URL to open when tapped (must be a valid URL)
    pub fn action_url(mut self, url: impl Into<String>) -> Self {
        self.action_url = Some(url.into());
        self
    }

    /// Sets the encryption password (will encrypt message field only)
    ///
    /// When set, the message will be encrypted using AES-128-CBC with:
    /// - SHA1-based key derivation
    /// - Random 16-byte IV per message
    /// - Custom Base64 encoding (+ → -, / → ., = → _)
    /// - PKCS7 padding
    ///
    /// Note: Only the message field is encrypted. Title, type, imageURL,
    /// actionURL, and tags remain unencrypted.
    pub fn encryption_password(mut self, password: impl Into<String>) -> Self {
        self.encryption_password = Some(password.into());
        self
    }

    /// Builds the notification, validating all fields
    pub fn build(self) -> Result<Notification> {
        let title = self
            .title
            .ok_or_else(|| Error::BuilderValidation("title is required".to_string()))?;

        let message = self
            .message
            .ok_or_else(|| Error::BuilderValidation("message is required".to_string()))?;

        // Validate title length
        if title.len() > 256 {
            return Err(Error::BuilderValidation(
                "title must be 256 characters or less".to_string(),
            ));
        }

        // Validate message length
        if message.len() > 4096 {
            return Err(Error::BuilderValidation(
                "message must be 4096 characters or less".to_string(),
            ));
        }

        // Validate tags count and individual tag lengths
        if let Some(ref tags) = self.tags {
            if tags.len() > 10 {
                return Err(Error::BuilderValidation(
                    "maximum 10 tags allowed".to_string(),
                ));
            }

            // Validate individual tag length (max 50 characters per tag)
            for (index, tag) in tags.iter().enumerate() {
                if tag.is_empty() {
                    return Err(Error::BuilderValidation(format!(
                        "tag at index {} cannot be empty",
                        index
                    )));
                }
                if tag.len() > 50 {
                    return Err(Error::BuilderValidation(format!(
                        "tag at index {} exceeds 50 characters (length: {})",
                        index,
                        tag.len()
                    )));
                }
            }
        }

        // Validate image_url is a valid URL
        if let Some(ref image_url) = self.image_url {
            Url::parse(image_url)
                .map_err(|e| Error::BuilderValidation(format!("invalid image_url: {}", e)))?;
        }

        // Validate action_url is a valid URL
        if let Some(ref action_url) = self.action_url {
            Url::parse(action_url)
                .map_err(|e| Error::BuilderValidation(format!("invalid action_url: {}", e)))?;
        }

        Ok(Notification {
            title,
            message,
            notification_type: self.notification_type,
            tags: self.tags,
            image_url: self.image_url,
            action_url: self.action_url,
            encryption_password: self.encryption_password,
            message_iv: None, // Will be set during send if encryption is used
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_new() {
        let notification = Notification::new("Test Title", "Test Message");
        assert_eq!(notification.title, "Test Title");
        assert_eq!(notification.message, "Test Message");
        assert!(notification.notification_type.is_none());
    }

    #[test]
    fn test_builder_simple() {
        let notification = Notification::builder()
            .title("Test Title")
            .message("Test Message")
            .build()
            .unwrap();

        assert_eq!(notification.title, "Test Title");
        assert_eq!(notification.message, "Test Message");
    }

    #[test]
    fn test_builder_full() {
        let notification = Notification::builder()
            .title("Deploy Complete")
            .message("v1.2.3 deployed")
            .notification_type("deployment")
            .tags(vec!["prod".to_string(), "release".to_string()])
            .image_url("https://example.com/img.png")
            .action_url("https://example.com/deploy/123")
            .build()
            .unwrap();

        assert_eq!(notification.title, "Deploy Complete");
        assert_eq!(notification.message, "v1.2.3 deployed");
        assert_eq!(
            notification.notification_type,
            Some("deployment".to_string())
        );
        assert_eq!(
            notification.tags,
            Some(vec!["prod".to_string(), "release".to_string()])
        );
        assert_eq!(
            notification.image_url,
            Some("https://example.com/img.png".to_string())
        );
        assert_eq!(
            notification.action_url,
            Some("https://example.com/deploy/123".to_string())
        );
    }

    #[test]
    fn test_builder_add_tag() {
        let notification = Notification::builder()
            .title("Test")
            .message("Message")
            .add_tag("tag1")
            .add_tag("tag2")
            .build()
            .unwrap();

        assert_eq!(
            notification.tags,
            Some(vec!["tag1".to_string(), "tag2".to_string()])
        );
    }

    #[test]
    fn test_builder_missing_title() {
        let result = Notification::builder().message("Test Message").build();

        assert!(result.is_err());
        assert!(matches!(result, Err(Error::BuilderValidation(_))));
    }

    #[test]
    fn test_builder_missing_message() {
        let result = Notification::builder().title("Test Title").build();

        assert!(result.is_err());
        assert!(matches!(result, Err(Error::BuilderValidation(_))));
    }

    #[test]
    fn test_builder_title_too_long() {
        let long_title = "a".repeat(257);
        let result = Notification::builder()
            .title(long_title)
            .message("Test")
            .build();

        assert!(result.is_err());
        assert!(matches!(result, Err(Error::BuilderValidation(_))));
    }

    #[test]
    fn test_builder_message_too_long() {
        let long_message = "a".repeat(4097);
        let result = Notification::builder()
            .title("Test")
            .message(long_message)
            .build();

        assert!(result.is_err());
        assert!(matches!(result, Err(Error::BuilderValidation(_))));
    }

    #[test]
    fn test_builder_too_many_tags() {
        let tags: Vec<String> = (0..11).map(|i| format!("tag{}", i)).collect();
        let result = Notification::builder()
            .title("Test")
            .message("Message")
            .tags(tags)
            .build();

        assert!(result.is_err());
        assert!(matches!(result, Err(Error::BuilderValidation(_))));
    }

    #[test]
    fn test_json_serialization() {
        let notification = Notification::new("Test", "Message");
        let json = serde_json::to_string(&notification).unwrap();

        assert!(json.contains("Test"));
        assert!(json.contains("Message"));
    }

    #[test]
    fn test_json_serialization_with_type() {
        let notification = Notification::builder()
            .title("Test")
            .message("Message")
            .notification_type("info")
            .build()
            .unwrap();

        let json = serde_json::to_string(&notification).unwrap();
        assert!(json.contains("\"type\":\"info\""));
    }

    #[test]
    fn test_valid_image_url() {
        let result = Notification::builder()
            .title("Test")
            .message("Message")
            .image_url("https://example.com/image.png")
            .build();

        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_image_url() {
        let result = Notification::builder()
            .title("Test")
            .message("Message")
            .image_url("not-a-valid-url")
            .build();

        assert!(result.is_err());
        assert!(matches!(result, Err(Error::BuilderValidation(_))));
        if let Err(Error::BuilderValidation(msg)) = result {
            assert!(msg.contains("invalid image_url"));
        }
    }

    #[test]
    fn test_valid_action_url() {
        let result = Notification::builder()
            .title("Test")
            .message("Message")
            .action_url("https://example.com/action")
            .build();

        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_action_url() {
        let result = Notification::builder()
            .title("Test")
            .message("Message")
            .action_url("not a url at all")
            .build();

        assert!(result.is_err());
        assert!(matches!(result, Err(Error::BuilderValidation(_))));
        if let Err(Error::BuilderValidation(msg)) = result {
            assert!(msg.contains("invalid action_url"));
        }
    }

    #[test]
    fn test_tag_too_long() {
        let long_tag = "a".repeat(51);
        let result = Notification::builder()
            .title("Test")
            .message("Message")
            .tags(vec![long_tag])
            .build();

        assert!(result.is_err());
        assert!(matches!(result, Err(Error::BuilderValidation(_))));
        if let Err(Error::BuilderValidation(msg)) = result {
            assert!(msg.contains("exceeds 50 characters"));
        }
    }

    #[test]
    fn test_empty_tag() {
        let result = Notification::builder()
            .title("Test")
            .message("Message")
            .tags(vec!["valid".to_string(), "".to_string()])
            .build();

        assert!(result.is_err());
        assert!(matches!(result, Err(Error::BuilderValidation(_))));
        if let Err(Error::BuilderValidation(msg)) = result {
            assert!(msg.contains("cannot be empty"));
        }
    }

    #[test]
    fn test_valid_tags_at_max_length() {
        let max_length_tag = "a".repeat(50);
        let result = Notification::builder()
            .title("Test")
            .message("Message")
            .tags(vec![max_length_tag])
            .build();

        assert!(result.is_ok());
    }

    #[test]
    fn test_multiple_valid_tags() {
        let result = Notification::builder()
            .title("Test")
            .message("Message")
            .tags(vec!["tag1".to_string(), "tag2".to_string(), "a".repeat(50)])
            .build();

        assert!(result.is_ok());
    }
}
