# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `Client` and `ClientBuilder`: an `Arc`-backed async client over a
  `reqwest-middleware` stack, with `x-gladia-key` authentication, exponential-backoff
  retries, an optional per-request timeout, and custom headers.
- `Client::prerecorded()`, covering the pre-recorded transcription endpoints:
  `upload`, `upload_url`, `init`, `get`, `list`, `delete`, and `file`.
- `PreRecorded::transcribe` and `transcribe_url`, which upload, submit, and poll in
  one call.
- `PreRecorded::submit` and `job`, returning a `JobHandle` that polls to completion
  with `wait` / `wait_with`, or `into_result` to treat a failed transcription as an
  error. A handle can be rebuilt from a stored id, since jobs outlive the process
  that submitted them.
- `TranscriptionRequest`, which sets each feature flag alongside its config so the
  two cannot disagree, with `*_default` forms for the features whose config is
  entirely optional.
- `ListQuery`, for filtering and paginating `list`.
- `Client::live()` behind the `live` feature: `start` opens a session, which is a
  `Stream` of typed `Message`s with audio sent through an `AudioSender` taken from
  it, so audio and transcripts can be driven from separate tasks. Also `init` and
  `connect` for the two steps separately, and `get` / `list` / `delete` / `file` for
  finished sessions.
- `gladia::model`: wire types for the whole API, generated from the vendored OpenAPI
  document with [`typify`]. `cargo xtask codegen` regenerates them, and CI fails when
  they drift from the spec.
- `Error`, which parses Gladia's JSON error envelope, including `validation_errors`.
- Feature flags: `rustls-tls` (default), `native-tls`, `live`, and `tracing`.

[`typify`]: https://github.com/oxidecomputer/typify

[Unreleased]: https://github.com/martsokha/gladia/compare/HEAD
