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
