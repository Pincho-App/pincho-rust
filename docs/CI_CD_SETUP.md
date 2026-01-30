# CI/CD Setup Guide for Pincho Rust SDK

This guide covers the Google Cloud Build pipeline for automated testing and publishing to crates.io.

## Infrastructure (Already Configured)

The CI/CD infrastructure is managed via Terraform in the `frontend` repository:

- **Location**: `frontend/terraform/modules/cloudbuild/main.tf`
- **Repository link**: pincho-rust → GitLab
- **Triggers**: Main branch (CI) + Release tags (publish)
- **Secrets**: `crates-io-token` and `pincho-token` access configured

No manual setup required - just push code!

## Pipeline Flow

```
Push to main:
  fetch → build → test → coverage (80%) → clippy → fmt → docs → package

Push v*.*.* tag:
  ... same as above ... → publish to crates.io → Pincho notification
```

## Testing Locally

### Run All Pipeline Steps

```bash
# Fetch dependencies
cargo fetch --locked

# Build
cargo build --release --locked

# Test
cargo test --locked

# Coverage (requires cargo-tarpaulin)
cargo install cargo-tarpaulin
cargo tarpaulin --out Stdout

# Lint
cargo clippy --all-targets -- -D warnings

# Format check
cargo fmt -- --check

# Documentation
cargo doc --no-deps

# Package
cargo package --locked --allow-dirty
```

### Test in Docker (Closer to Cloud Build)

```bash
docker run --rm -v "$(pwd)":/app -w /app rust:1.83 bash -c "
  cargo fetch --locked &&
  cargo build --release --locked &&
  cargo test --locked &&
  cargo clippy --all-targets -- -D warnings &&
  cargo fmt -- --check &&
  cargo package --locked --allow-dirty
"
```

## Publishing a Release

1. **Update version** in `Cargo.toml`
2. **Commit changes**: `git commit -am "chore: bump version to X.Y.Z"`
3. **Create tag**: `git tag vX.Y.Z`
4. **Push**: `git push origin main --tags`

Cloud Build will automatically:
- Run all tests and checks
- Publish to crates.io
- Send a notification via Pincho

## Troubleshooting

### Secret Access Denied

```
Error: Secret "crates-io-token" is not accessible
```

Ensure Cloud Build service account has `secretAccessor` role on the secret.

### Coverage Below Threshold

```
ERROR: Coverage 79.5% is below 80% threshold
```

Add more tests to increase coverage, particularly for uncovered code paths in `client.rs`.

### Package Already Exists on crates.io

```
error: crate version `1.0.0` is already uploaded
```

Bump the version in `Cargo.toml` before tagging a new release.

### Build Timeout

Default timeout is 600s (10 minutes). If builds consistently timeout, consider:
- Using a larger machine type
- Optimizing test suite
- Caching dependencies (not currently implemented)

## Monitoring

- **Cloud Build History**: Console → Cloud Build → History
- **Build Logs**: Click on any build for detailed logs
- **Artifacts**: Stored in `gs://${PROJECT_ID}_cloudbuild/artifacts/rust-sdk/`

## Security Notes

- Never commit crates.io tokens to the repository
- Rotate tokens periodically in Secret Manager
- Review Cloud Build service account permissions
- Monitor for unexpected publish attempts

## Links

- [Cloud Build Documentation](https://cloud.google.com/build/docs)
- [Secret Manager](https://cloud.google.com/secret-manager/docs)
- [crates.io API Tokens](https://crates.io/settings/tokens)
- [Cargo Publish](https://doc.rust-lang.org/cargo/reference/publishing.html)
