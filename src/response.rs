use serde::{Deserialize, Serialize};

/// Response from the WirePusher API
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotificationResponse {
    /// Response status (e.g., "success")
    pub status: String,

    /// Response message
    pub message: String,
}

impl NotificationResponse {
    /// Returns true if the notification was sent successfully
    pub fn is_success(&self) -> bool {
        self.status == "success"
    }
}

/// Error response from the WirePusher API (nested format)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ErrorResponse {
    /// Response status (e.g., "error")
    pub status: String,

    /// Nested error details
    pub error: ErrorDetail,
}

/// Nested error details from the WirePusher API
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ErrorDetail {
    /// Error type (e.g., "validation_error", "authentication_error")
    #[serde(rename = "type")]
    pub error_type: String,

    /// Error code (e.g., "invalid_parameter", "missing_field")
    pub code: String,

    /// Human-readable error message
    pub message: String,

    /// Parameter that caused the error (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_success() {
        let response = NotificationResponse {
            status: "success".to_string(),
            message: "Notification sent".to_string(),
        };
        assert!(response.is_success());
    }

    #[test]
    fn test_is_not_success() {
        let response = NotificationResponse {
            status: "error".to_string(),
            message: "Failed".to_string(),
        };
        assert!(!response.is_success());
    }

    #[test]
    fn test_json_serialization() {
        let response = NotificationResponse {
            status: "success".to_string(),
            message: "Notification sent".to_string(),
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("success"));
        assert!(json.contains("Notification sent"));
    }

    #[test]
    fn test_json_deserialization() {
        let json = r#"{"status":"success","message":"Notification sent"}"#;
        let response: NotificationResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.status, "success");
        assert_eq!(response.message, "Notification sent");
        assert!(response.is_success());
    }
}
