# Security Policy

## Supported Versions

We release patches for security vulnerabilities in the following versions:

| Version | Supported          |
| ------- | ------------------ |
| 1.x.x   | :white_check_mark: |
| < 1.0   | :x:                |

## Reporting a Vulnerability

The Pincho team takes security bugs seriously. We appreciate your efforts to responsibly disclose your findings.

### How to Report

**Please do NOT report security vulnerabilities through public GitLab issues.**

Instead, please report security vulnerabilities via email to:

**security@pincho.app**

### What to Include

To help us triage and fix the issue quickly, please include:

1. **Type of vulnerability** (e.g., authentication bypass, injection, etc.)
2. **Full paths** of source files related to the vulnerability
3. **Location** of the affected source code (tag/branch/commit or direct URL)
4. **Step-by-step instructions** to reproduce the issue
5. **Proof-of-concept or exploit code** (if possible)
6. **Impact** of the vulnerability (what an attacker could achieve)
7. **Any mitigating factors** or workarounds you've identified

### What to Expect

After you submit a report:

1. **Acknowledgment** - We'll acknowledge receipt within 48 hours
2. **Assessment** - We'll assess the vulnerability and determine severity
3. **Updates** - We'll provide regular updates (at least every 7 days)
4. **Fix Timeline** - We aim to release fixes for:
   - **Critical** vulnerabilities: Within 7 days
   - **High** vulnerabilities: Within 14 days
   - **Medium** vulnerabilities: Within 30 days
   - **Low** vulnerabilities: Next regular release

5. **Disclosure** - We'll coordinate with you on public disclosure timing
6. **Credit** - We'll credit you in the security advisory (unless you prefer to remain anonymous)

## Security Best Practices

### For Users

When using the Pincho Rust Client Library:

1. **Keep the SDK updated** to the latest version
2. **Never commit credentials** to version control
3. **Use environment variables** for sensitive configuration
4. **Validate input** before sending to the SDK
5. **Handle errors gracefully** without exposing sensitive information
6. **Use HTTPS** for all network communication
7. **Limit token scope** to minimum required permissions

### Credential Management

```rust
// ❌ Bad - Hardcoded credentials
let client = Client::new("abc12345", "device_id")?;

// ✅ Good - Environment variables
use std::env;
let token = env::var("PINCHO_TOKEN")?;
let device_id = env::var("PINCHO_DEVICE_ID")?;
let client = Client::new(token, device_id)?;
```

### Error Handling

```rust
// ❌ Bad - Exposes sensitive information
match client.send("Title", "Message").await {
    Err(e) => println!("Error: {:?}", e), // May log tokens
    Ok(_) => println!("Success"),
}

// ✅ Good - Safe error handling
match client.send("Title", "Message").await {
    Ok(_) => println!("Notification sent"),
    Err(Error::Authentication { .. }) => {
        eprintln!("Authentication failed - check credentials");
    }
    Err(Error::Validation { message, .. }) => {
        eprintln!("Validation error: {}", message);
    }
    Err(_) => eprintln!("Notification failed"),
}
```

### Input Validation

```rust
// ❌ Bad - No validation
async fn handle_request(title: String, message: String) {
    client.send(&title, &message).await.ok();
}

// ✅ Good - Validate input
async fn handle_request(title: String, message: String) -> Result<()> {
    if title.is_empty() || message.is_empty() {
        return Err("Missing required fields".into());
    }

    if title.len() > 256 || message.len() > 4096 {
        return Err("Content too long".into());
    }

    client.send(&title, &message).await?;
    Ok(())
}
```

### Context Timeouts

```rust
use tokio::time::{timeout, Duration};

// ❌ Bad - No timeout
client.send("Title", "Message").await?;

// ✅ Good - Use timeout
timeout(
    Duration::from_secs(10),
    client.send("Title", "Message")
).await??;
```

## Known Security Considerations

### API Token Security

- Tokens are transmitted in API requests and should be kept confidential
- Tokens are stored in plaintext by the SDK (secure storage is the user's responsibility)
- Compromised tokens can be used to send notifications as your user
- Rotate tokens regularly as a security best practice

### Network Communication

- All communication with Pincho API is over HTTPS
- The SDK uses reqwest which respects system-level TLS/SSL settings
- Certificate validation is handled by the Rust runtime
- Minimum TLS 1.2 is enforced by default

### Dependencies

The SDK has minimal runtime dependencies to reduce supply chain risks:
- `reqwest` - Well-maintained HTTP client with security focus
- `tokio` - Industry-standard async runtime
- `serde`/`serde_json` - Widely-used serialization library
- `thiserror` - Error handling
- `aes`, `cbc`, `sha1` - Cryptography (encryption support)

We monitor dependencies for known vulnerabilities and update promptly.

### Memory Safety

- Rust's ownership system prevents buffer overflows and memory corruption
- Borrow checker prevents use-after-free vulnerabilities
- Type safety prevents many common programming errors
- No unsafe code in the library (except in vetted dependencies)

## Vulnerability Disclosure Process

When we receive a security bug report:

1. **Confirm the vulnerability** and determine affected versions
2. **Develop and test a fix** for all supported versions
3. **Prepare security advisory** with:
   - Description of the vulnerability
   - Affected versions
   - Fixed versions
   - Workarounds (if any)
   - Credit to reporter
4. **Release patched versions**
5. **Publish security advisory** on GitLab
6. **Notify users** via:
   - GitLab security advisory
   - Project README update
   - crates.io documentation

## Security Audit History

| Date | Type | Findings | Status |
|------|------|----------|--------|
| TBD  | TBD  | TBD      | TBD    |

## Security Hall of Fame

We thank the following individuals for responsibly disclosing security vulnerabilities:

- (None yet)

## Resources

- [Rust Security Best Practices](https://anssi-fr.github.io/rust-guide/)
- [OWASP API Security Top 10](https://owasp.org/www-project-api-security/)
- [RustSec Advisory Database](https://rustsec.org/)

## Questions?

For security-related questions that aren't reporting vulnerabilities:

- Email: security@pincho.app
- General questions: support@pincho.app

Thank you for helping keep Pincho and its users safe!
