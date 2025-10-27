# Security Policy

## Supported Versions

We release patches for security vulnerabilities. Which versions are eligible for receiving such patches depends on the CVSS v3.0 Rating:

| Version | Supported          |
| ------- | ------------------ |
| 1.x.x   | :white_check_mark: |

## Reporting a Vulnerability

If you discover a security vulnerability within the WirePusher Rust SDK, please send an email to security@wirepusher.dev. All security vulnerabilities will be promptly addressed.

**Please do not report security vulnerabilities through public GitHub/GitLab issues.**

When reporting a vulnerability, please include:

- Type of issue (e.g., buffer overflow, SQL injection, cross-site scripting, etc.)
- Full paths of source file(s) related to the manifestation of the issue
- The location of the affected source code (tag/branch/commit or direct URL)
- Any special configuration required to reproduce the issue
- Step-by-step instructions to reproduce the issue
- Proof-of-concept or exploit code (if possible)
- Impact of the issue, including how an attacker might exploit it

We will acknowledge receipt of your vulnerability report and send you regular updates about our progress. If you have suggestions for how to address the vulnerability, please include them in your report.

## Disclosure Policy

When we receive a security bug report, we will:

1. Confirm the problem and determine the affected versions
2. Audit code to find any similar problems
3. Prepare fixes for all supported releases
4. Release new versions as soon as possible

## Security Best Practices

When using this SDK:

1. **Never hardcode credentials** - Use environment variables or secure configuration management
2. **Keep dependencies updated** - Regularly update the SDK and its dependencies
3. **Use HTTPS** - The SDK defaults to HTTPS for all API calls
4. **Protect your API tokens** - Treat them like passwords
5. **Use environment-specific tokens** - Don't use production tokens in development
6. **Rotate tokens regularly** - Generate new tokens periodically
7. **Monitor for suspicious activity** - Check your WirePusher dashboard regularly

## Security Features

This SDK includes:

- TLS/SSL encryption for all API communications
- No logging of sensitive data (tokens, user IDs)
- Connection timeout protection (30 seconds default)
- Comprehensive error handling without exposing internal details
- Type-safe API to prevent common programming errors
