# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Initial project scaffold: `Client` and `ClientBuilder` over a `reqwest-middleware`
  stack with exponential-backoff retries, an optional per-request timeout, and custom
  headers.
- `x-gladia-key` API key authentication, applied via default headers and marked
  sensitive.
- `Error` / `Result` types parsing Gladia's JSON error envelope, including
  `validation_errors`.
- Feature flags: `rustls-tls` (default), `native-tls`, `live`, `tracing`.

[Unreleased]: https://github.com/martsokha/gladia/compare/HEAD
