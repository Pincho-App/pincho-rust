# CI/CD Guide for Pincho Rust SDK

This guide covers the GitHub Actions workflow (`.github/workflows/ci.yml`) that tests the crate and publishes it to crates.io.

## Pipeline Flow

| Trigger | Jobs |
|---------|------|
| Push to `main`, pull request | Lint (`cargo fmt --check`, `cargo clippy -D warnings`) → Test → Build (`cargo doc`, `cargo package`) |
| Tag `vX.Y.Z` | Same as above → Publish to crates.io → Pincho release notification |

Secrets come from GitHub Actions organization secrets (`CRATES_IO_TOKEN` scoped to this repository, `PINCHO_TOKEN` shared). Both are written from GCP Secret Manager by OpenTofu in the `pincho` repository (`infra/modules/github-ci`); nothing is configured by hand in GitHub.

## Testing Locally

Run the same steps the workflow runs:

```bash
cargo fetch --locked
cargo build --release --locked
cargo test --locked
cargo clippy --all-targets -- -D warnings
cargo fmt -- --check
cargo doc --no-deps
cargo package --locked --allow-dirty
```

## Publishing a Release

1. Update the version in `Cargo.toml`
2. Commit: `git commit -am "Bump version to X.Y.Z"`
3. Tag: `git tag vX.Y.Z`
4. Push: `git push origin main --tags`

The workflow then runs all checks, publishes to crates.io, and sends a notification via Pincho. Follow it with `gh run watch` or in the repository's Actions tab.

## Troubleshooting

### Package Already Exists on crates.io

```
error: crate version `1.0.0` is already uploaded
```

Bump the version in `Cargo.toml` before tagging a new release.

### Publish Rejected (401/403)

The `CRATES_IO_TOKEN` organization secret is missing, expired, or not scoped to this repository. Rotate the token on crates.io, add a new version of the `crates-io-token` secret in Secret Manager, and re-apply the `github-ci` module in the `pincho` repository.

## Security Notes

- Never commit crates.io tokens to the repository
- Rotate tokens periodically in Secret Manager; GitHub only ever holds copies
- Monitor for unexpected publish attempts

## Links

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [crates.io API Tokens](https://crates.io/settings/tokens)
- [Cargo Publish](https://doc.rust-lang.org/cargo/reference/publishing.html)
