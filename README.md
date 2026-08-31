# gladia

[![Build](https://img.shields.io/github/actions/workflow/status/martsokha/gladia/build.yml?branch=main&label=build%20%26%20test&style=flat-square)](https://github.com/martsokha/gladia/actions/workflows/build.yml)
[![Crate](https://img.shields.io/crates/v/gladia.svg?style=flat-square)](https://crates.io/crates/gladia)
[![Docs](https://img.shields.io/docsrs/gladia?style=flat-square)](https://docs.rs/gladia)

An unofficial async Rust client for the [Gladia] speech-to-text API ([docs][gladia-docs]).

Gladia transcribes audio in two modes: **pre-recorded**, where a file is uploaded or
referenced by URL and transcribed asynchronously, and **live**, where audio is streamed
over a WebSocket and transcripts arrive as the session runs. This crate wraps both over
one `Arc`-backed client.

## Status

Early development: the client, configuration, and error surface are in place, and the
endpoint modules are being added. The API is not yet stable.

## Usage

Add the dependency:

```toml
[dependencies]
gladia = "0.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

The minimum supported Rust version (MSRV) is **1.91**.

```rust,no_run
use gladia::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::builder()
        .with_api_key(std::env::var("GLADIA_API_KEY").expect("GLADIA_API_KEY"))
        .build()?;

    println!("{}", client.base_url());
    Ok(())
}
```

An API key is required — issue one in the [Gladia dashboard]. It is sent as the
`x-gladia-key` header on every request, and marked sensitive so it is redacted from
header debug output. See [`examples/`](examples/) for runnable examples.

## Features

- **Resilient transport**: exponential-backoff retries via `reqwest-middleware`, an
  optional per-request timeout, custom headers, and a cheap-to-clone `Arc`-backed
  client that shares one connection pool across clones.
- **Typed errors**: Gladia's JSON error envelope is parsed into `Error::Api`, keeping
  the status, message, error label, and any field-level `validation_errors`.
- **Escape hatches**: `as_client` / `as_client_with_middleware` expose the underlying
  `reqwest` client, pre-authenticated, for requests this crate doesn't yet model.

These are gated by feature flags:

- `rustls-tls` *(default)*: HTTPS via Rustls.
- `native-tls`: HTTPS via the platform-native TLS stack.
- `live`: live transcription over WebSocket.
- `tracing`: `#[tracing::instrument]` spans on request methods.

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for release notes and version history.

## License

Licensed under the [MIT License](LICENSE.txt).

[Gladia]: https://www.gladia.io
[gladia-docs]: https://docs.gladia.io
[Gladia dashboard]: https://app.gladia.io
