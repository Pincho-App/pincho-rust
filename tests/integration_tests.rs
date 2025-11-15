use wiremock::{
    matchers::{method, path, query_param},
    Mock, MockServer, ResponseTemplate,
};
use wirepusher::{Client, Error, Notification};

#[tokio::test]
async fn test_send_simple_notification_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/send"))
        .and(query_param("type", "wirepusher"))
        .and(query_param("token", "test_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "success",
            "message": "Notification sent"
        })))
        .mount(&mock_server)
        .await;

    let base_url = format!("{}/send", mock_server.uri());
    let client = Client::with_base_url("test_token", "test_device", &base_url).unwrap();

    let response = client.send("Test Title", "Test Message").await.unwrap();

    assert!(response.is_success());
    assert_eq!(response.status, "success");
    assert_eq!(response.message, "Notification sent");
}

#[tokio::test]
async fn test_send_notification_with_all_options() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/send"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "success",
            "message": "Notification sent with options"
        })))
        .mount(&mock_server)
        .await;

    let base_url = format!("{}/send", mock_server.uri());
    let client = Client::with_base_url("test_token", "test_device", &base_url).unwrap();

    let notification = Notification::builder()
        .title("Deploy Complete")
        .message("v1.2.3 deployed")
        .notification_type("deployment")
        .tags(vec!["prod".to_string(), "release".to_string()])
        .image_url("https://example.com/img.png")
        .action_url("https://example.com/deploy/123")
        .build()
        .unwrap();

    let response = client.send_notification(notification).await.unwrap();

    assert!(response.is_success());
    assert_eq!(response.message, "Notification sent with options");
}

#[tokio::test]
async fn test_authentication_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/send"))
        .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
            "status": "error",
            "error": {
                "type": "authentication_error",
                "code": "invalid_token",
                "message": "Token is invalid"
            }
        })))
        .mount(&mock_server)
        .await;

    let base_url = format!("{}/send", mock_server.uri());
    let client = Client::with_base_url("invalid_token", "test_device", &base_url).unwrap();

    let result = client.send("Test", "Message").await;

    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.is_authentication_error());
    assert_eq!(error.status_code(), Some(401));

    // Verify error message contains code
    let error_msg = error.to_string();
    assert!(error_msg.contains("Token is invalid"));
    assert!(error_msg.contains("[invalid_token]"));
}

#[tokio::test]
async fn test_validation_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/send"))
        .respond_with(ResponseTemplate::new(400).set_body_json(serde_json::json!({
            "status": "error",
            "error": {
                "type": "validation_error",
                "code": "invalid_parameter",
                "message": "Title is required",
                "param": "title"
            }
        })))
        .mount(&mock_server)
        .await;

    let base_url = format!("{}/send", mock_server.uri());
    let client = Client::with_base_url("test_token", "test_device", &base_url).unwrap();

    let result = client.send("", "").await;

    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.is_validation_error());
    assert_eq!(error.status_code(), Some(400));

    // Verify error message contains code and param
    let error_msg = error.to_string();
    assert!(error_msg.contains("Title is required"));
    assert!(error_msg.contains("[invalid_parameter]"));
    assert!(error_msg.contains("(parameter: title)"));
}

#[tokio::test]
async fn test_rate_limit_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/send"))
        .respond_with(ResponseTemplate::new(429).set_body_json(serde_json::json!({
            "status": "error",
            "error": {
                "type": "rate_limit_error",
                "code": "rate_limit_exceeded",
                "message": "Rate limit exceeded"
            }
        })))
        .mount(&mock_server)
        .await;

    let base_url = format!("{}/send", mock_server.uri());
    let client = Client::with_base_url("test_token", "test_device", &base_url).unwrap();

    let result = client.send("Test", "Message").await;

    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.is_rate_limit_error());
    assert_eq!(error.status_code(), Some(429));

    // Verify error message contains code
    let error_msg = error.to_string();
    assert!(error_msg.contains("Rate limit exceeded"));
    assert!(error_msg.contains("[rate_limit_exceeded]"));
}

#[tokio::test]
async fn test_forbidden_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/send"))
        .respond_with(ResponseTemplate::new(403).set_body_json(serde_json::json!({
            "status": "error",
            "error": {
                "type": "authentication_error",
                "code": "forbidden",
                "message": "Access denied"
            }
        })))
        .mount(&mock_server)
        .await;

    let base_url = format!("{}/send", mock_server.uri());
    let client = Client::with_base_url("test_token", "test_device", &base_url).unwrap();

    let result = client.send("Test", "Message").await;

    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.is_authentication_error());
    assert_eq!(error.status_code(), Some(403));
}

#[tokio::test]
async fn test_not_found_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/send"))
        .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
            "status": "error",
            "error": {
                "type": "validation_error",
                "code": "not_found",
                "message": "Resource not found"
            }
        })))
        .mount(&mock_server)
        .await;

    let base_url = format!("{}/send", mock_server.uri());
    let client = Client::with_base_url("test_token", "test_device", &base_url).unwrap();

    let result = client.send("Test", "Message").await;

    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.is_validation_error());
    assert_eq!(error.status_code(), Some(404));
}

#[tokio::test]
async fn test_server_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/send"))
        .respond_with(ResponseTemplate::new(500).set_body_json(serde_json::json!({
            "status": "error",
            "error": {
                "type": "server_error",
                "code": "internal_error",
                "message": "Internal server error"
            }
        })))
        .mount(&mock_server)
        .await;

    let base_url = format!("{}/send", mock_server.uri());
    let client = Client::with_base_url("test_token", "test_device", &base_url).unwrap();

    let result = client.send("Test", "Message").await;

    assert!(result.is_err());
    let error = result.unwrap_err();
    assert_eq!(error.status_code(), Some(500));
}

#[test]
fn test_client_creation_validation() {
    // Empty token
    let result = Client::new("", "device_id");
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), Error::InvalidConfig(_)));

    // Empty device ID
    let result = Client::new("token", "");
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), Error::InvalidConfig(_)));

    // Valid client
    let result = Client::new("token", "device_id");
    assert!(result.is_ok());
}

#[test]
fn test_notification_builder_validation() {
    // Missing title
    let result = Notification::builder().message("Test").build();
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), Error::BuilderValidation(_)));

    // Missing message
    let result = Notification::builder().title("Test").build();
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), Error::BuilderValidation(_)));

    // Title too long
    let long_title = "a".repeat(257);
    let result = Notification::builder()
        .title(long_title)
        .message("Test")
        .build();
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), Error::BuilderValidation(_)));

    // Message too long
    let long_message = "a".repeat(4097);
    let result = Notification::builder()
        .title("Test")
        .message(long_message)
        .build();
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), Error::BuilderValidation(_)));

    // Too many tags
    let tags: Vec<String> = (0..11).map(|i| format!("tag{}", i)).collect();
    let result = Notification::builder()
        .title("Test")
        .message("Message")
        .tags(tags)
        .build();
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), Error::BuilderValidation(_)));

    // Valid notification
    let result = Notification::builder()
        .title("Test")
        .message("Message")
        .build();
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_concurrent_requests() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/send"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "success",
            "message": "Notification sent"
        })))
        .mount(&mock_server)
        .await;

    let base_url = format!("{}/send", mock_server.uri());
    let client = Client::with_base_url("test_token", "test_device", &base_url).unwrap();

    // Send multiple concurrent requests
    let futures: Vec<_> = (0..10)
        .map(|i| {
            let client = client.clone();
            async move {
                client
                    .send(format!("Title {}", i), format!("Message {}", i))
                    .await
            }
        })
        .collect();

    let results = futures::future::join_all(futures).await;

    // All should succeed
    for result in results {
        assert!(result.is_ok());
        assert!(result.unwrap().is_success());
    }
}
