use wiremock::{
    matchers::{header, method, path},
    Mock, MockServer, ResponseTemplate,
};
use wirepusher::{Client, Error, Notification};

#[tokio::test]
async fn test_send_simple_notification_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/send"))
        .and(header("authorization", "Bearer test_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "success",
            "message": "Notification sent"
        })))
        .mount(&mock_server)
        .await;

    let client = Client::with_base_url("test_token", mock_server.uri()).unwrap();

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
        .and(header("authorization", "Bearer test_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "success",
            "message": "Notification sent with options"
        })))
        .mount(&mock_server)
        .await;

    let client = Client::with_base_url("test_token", mock_server.uri()).unwrap();

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

    let mut client = Client::new("invalid_token").unwrap();
    client.set_base_url(mock_server.uri());
    client.set_max_retries(0); // No retries for this test

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

    let client = Client::with_base_url("test_token", mock_server.uri()).unwrap();

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
async fn test_rate_limit_error_no_retry() {
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
        .expect(1) // Only called once (no retries)
        .mount(&mock_server)
        .await;

    // Create client with 0 retries
    let mut client = Client::new("test_token").unwrap();
    client.set_base_url(mock_server.uri());
    client.set_max_retries(0); // Disable retries for this test

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

    let client = Client::with_base_url("test_token", mock_server.uri()).unwrap();

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

    let client = Client::with_base_url("test_token", mock_server.uri()).unwrap();

    let result = client.send("Test", "Message").await;

    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.is_validation_error());
    assert_eq!(error.status_code(), Some(404));
}

#[tokio::test]
async fn test_server_error_no_retry() {
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
        .expect(1) // Only one call with no retries
        .mount(&mock_server)
        .await;

    let mut client = Client::new("test_token").unwrap();
    client.set_base_url(mock_server.uri());
    client.set_max_retries(0); // Disable retries for this test

    let result = client.send("Test", "Message").await;

    assert!(result.is_err());
    let error = result.unwrap_err();
    assert_eq!(error.status_code(), Some(500));
    assert!(error.is_retryable()); // Server errors are retryable
}

#[test]
fn test_client_creation_validation() {
    // Empty token
    let result = Client::new("");
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), Error::InvalidConfig(_)));

    // Valid client
    let result = Client::new("token");
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

    let client = Client::with_base_url("test_token", mock_server.uri()).unwrap();

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

#[tokio::test]
async fn test_rate_limit_info_parsing() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/send"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({
                    "status": "success",
                    "message": "Notification sent"
                }))
                .insert_header("RateLimit-Limit", "100")
                .insert_header("RateLimit-Remaining", "99")
                .insert_header("RateLimit-Reset", "1234567890"),
        )
        .mount(&mock_server)
        .await;

    let client = Client::with_base_url("test_token", mock_server.uri()).unwrap();

    let response = client.send("Test", "Message").await.unwrap();
    assert!(response.is_success());

    // Check rate limit info
    let rate_limit = client.get_last_rate_limit();
    assert!(rate_limit.is_some());
    let rate_limit = rate_limit.unwrap();
    assert_eq!(rate_limit.limit, 100);
    assert_eq!(rate_limit.remaining, 99);
    assert_eq!(rate_limit.reset, 1234567890);
}

#[tokio::test]
async fn test_notifai_endpoint() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/notifai"))
        .and(header("authorization", "Bearer test_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "success",
            "notification": {
                "title": "Deployment Complete",
                "message": "Version 2.1.3 has been deployed successfully",
                "type": "deployment"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = Client::with_base_url("test_token", mock_server.uri()).unwrap();

    let response = client
        .notifai("deployment finished v2.1.3", Some("deployment".to_string()))
        .await
        .unwrap();

    assert_eq!(response.status, "success");
    assert_eq!(response.notification.title, "Deployment Complete");
    assert!(response
        .notification
        .message
        .contains("deployed successfully"));
    assert_eq!(
        response.notification.notification_type,
        Some("deployment".to_string())
    );
}

#[test]
fn test_error_retryable() {
    // Rate limit errors are retryable
    let error = Error::RateLimit {
        message: "Rate limited".to_string(),
        status_code: 429,
    };
    assert!(error.is_retryable());

    // Server errors are retryable
    let error = Error::Api {
        message: "Server error".to_string(),
        status_code: 500,
    };
    assert!(error.is_retryable());

    // Authentication errors are not retryable
    let error = Error::Authentication {
        message: "Invalid token".to_string(),
        status_code: 401,
    };
    assert!(!error.is_retryable());

    // Validation errors are not retryable
    let error = Error::Validation {
        message: "Invalid params".to_string(),
        status_code: 400,
    };
    assert!(!error.is_retryable());
}
