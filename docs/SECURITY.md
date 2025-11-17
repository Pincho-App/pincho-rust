# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 2.x.x   | :white_check_mark: |
| 1.x.x   | :x:                |

## Reporting a Vulnerability

If you discover a security vulnerability, please report it via email to:

**security@wirepusher.dev**

Please include:
- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Any suggested fixes

We will respond within 48 hours and work with you to address the issue.

## Security Best Practices

### Token Management

- **Never hardcode tokens** in source code
- Use environment variables: `WIREPUSHER_TOKEN`
- Consider using secret managers in production
- Rotate tokens periodically

```rust
// Good: Environment variable
let client = Client::from_env()?;

// Good: Secret manager
let token = get_token_from_vault()?;
let client = Client::new(token)?;

// Bad: Hardcoded
let client = Client::new("hardcoded_token")?;
```

### Encryption Passwords

- Use strong passwords (minimum 12 characters)
- Store passwords securely (not in version control)
- Different passwords for different notification types

```rust
let password = std::env::var("ENCRYPTION_PASSWORD")?;
let notification = Notification::builder()
    .title("Alert")
    .message("Sensitive data")
    .encryption_password(password)
    .build()?;
```

### Network Security

- All communication uses HTTPS (TLS 1.2+)
- Certificate validation is enabled by default
- Connection pooling reduces connection overhead

### Dependency Security

Run `cargo audit` regularly to check for vulnerabilities:

```bash
cargo install cargo-audit
cargo audit
```

### Rate Limiting

- Monitor rate limit headers to avoid service disruption
- Implement your own rate limiting for critical paths
- Use the automatic retry mechanism responsibly

```rust
if let Some(rl) = client.get_last_rate_limit() {
    if rl.remaining == 0 {
        // Wait until reset before sending more
        let wait_time = rl.reset - current_timestamp();
        tokio::time::sleep(Duration::from_secs(wait_time)).await;
    }
}
```
